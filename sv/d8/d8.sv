module Day8 #(
    parameter ROMFILE,
    parameter int MAX_C,
    parameter int MAX_R,
    parameter int P2_CHECKERS
) (
    input bit rst,
    input bit clk,
    output logic [10:0] p1,
    output logic p1done,
    output logic [17:0] p2,
    output logic p2done
);
    localparam int ARR_LEN = (MAX_C + 1) * MAX_R;
    localparam int ADDR_BITS = $clog2(ARR_LEN + 1);

    // localparam int P1_CHECKERS = (MAX_C - 2) * (MAX_R - 2);
    localparam int P1_CHECKERS = 0;

    localparam int P1_READERS = P1_CHECKERS;
    localparam int P2_READERS = P2_CHECKERS * 5;
    localparam int READERS = P1_READERS + P2_READERS;

    logic [ADDR_BITS - 1:0] all_addrs [READERS - 1:0];
    logic [7:0] all_outs [READERS - 1:0];

    // logic [ADDR_BITS - 1:0] p1_addrs [P1_READERS - 1:0];
    // assign all_addrs[0 +: P1_READERS] = p1_addrs;
    // logic [7:0] p1_outs [P1_READERS - 1:0];
    // assign p1_outs = all_outs[0 +: P1_READERS];

    logic [ADDR_BITS - 1:0] p2_addrs [P2_READERS - 1:0];
    assign all_addrs[P1_READERS +: P2_READERS] = p2_addrs;
    logic [7:0] p2_outs [P2_READERS - 1:0];
    assign p2_outs = all_outs[P1_READERS +: P2_READERS];

    Day8Part2SyncInterface #(
        .ADDR_BITS(ADDR_BITS),
        .MAX_C(MAX_C),
        .MAX_R(MAX_R),
        .CHECKERS(P2_CHECKERS)
    ) d8p2si (.clk(clk));
    assign d8p2si.rst = rst;
    assign p2 = d8p2si.p2sol;
    assign p2done = d8p2si.completed;

    ManyHeadRom #(
        .DATA_BITS(8),
        .ADDR_BITS(ADDR_BITS),
        .MAX_ADDR(MAX_R * (MAX_C + 1)),
        .HEADS(READERS),
        .INIT_FILE(ROMFILE)
    ) mhr (
        .addrs(all_addrs),
        .outs(all_outs)
    );

    localparam int P2_SC_RSTVAL_INT = MAX_C + 2;
    localparam bit [ADDR_BITS - 1:0] P2_SC_RSTVAL = P2_SC_RSTVAL_INT[ADDR_BITS - 1:0];
    SyncCellInterface #(.ACTORS(P2_CHECKERS), .DATA_BITS(ADDR_BITS)) p2sci (.clk(clk));
    assign p2sci.rst = rst;
    assign d8p2si.ind_cell_locked = p2sci.is_locked;
    assign d8p2si.ind_cell_locked_to = p2sci.locked_to;
    assign d8p2si.from_ind_cell = p2sci.data_out;
    assign p2sci.lock_reqs = d8p2si.ind_cell_lock_reqs;
    assign p2sci.data_inputs = d8p2si.to_ind_cells;
    SyncCell #(.ACTORS(P2_CHECKERS), .DATA_BITS(ADDR_BITS), .RST_VAL(P2_SC_RSTVAL)) p2sc (
        .sci(p2sci.SyncCellPorts)
    );

    genvar i;
    generate
        for(i = 0; i < P2_CHECKERS; i += 1) begin: d8_gen_p2si_assigns
            assign p2_addrs[i * 5] = d8p2si.mhr_cs_addr[i];
            assign d8p2si.mhr_cs_out[i] = p2_outs[i * 5];
            assign p2_addrs[i * 5 + 1] = d8p2si.mhr_us_addr[i];
            assign d8p2si.mhr_us_out[i] = p2_outs[i * 5 + 1];
            assign p2_addrs[i * 5 + 2] = d8p2si.mhr_rs_addr[i];
            assign d8p2si.mhr_rs_out[i] = p2_outs[i * 5 + 2];
            assign p2_addrs[i * 5 + 3] = d8p2si.mhr_ds_addr[i];
            assign d8p2si.mhr_ds_out[i] = p2_outs[i * 5 + 3];
            assign p2_addrs[i * 5 + 4] = d8p2si.mhr_ls_addr[i];
            assign d8p2si.mhr_ls_out[i] = p2_outs[i * 5 + 4];
        end
    endgenerate

    Day8Part2Sync #(
        .ADDR_BITS(ADDR_BITS),
        .MAX_C(MAX_C),
        .MAX_R(MAX_R),
        .CHECKERS(P2_CHECKERS)
    ) d8p2s (
        .si(d8p2si.D8P2SPorts)
    );
endmodule

module Day8TL (
    input bit rst,
    input bit clk
);
    logic [10:0] p1;
    logic p1done;
    logic [17:0] p2;
    logic p2done;

    Day8 #(.ROMFILE("../inputs/d8-hex.txt"), .MAX_C(99), .MAX_R(99), .P2_CHECKERS(8)) d8 (
        .rst(~rst),
        .clk(clk),
        .p1(p1),
        .p1done(p1done),
        .p2(p2),
        .p2done(p2done)
    );
endmodule

`timescale 1ns/1ns
module Day8Sim ();
    logic rst;
    logic clk;
    logic [10:0] p1;
    logic p1done;
    logic [17:0] p2;
    logic p2done;

    Day8 #(
        .ROMFILE("/storage/projects/aoc-2022/inputs/d8-hex.txt"),
        .MAX_C(99),
        .MAX_R(99),
        // .ROMFILE("/storage/projects/aoc-2022/inputs/d8test-hex.txt"),
        // .MAX_C(5),
        // .MAX_R(5),
        .P2_CHECKERS(8)
    ) d8 (
        .rst(rst),
        .clk(clk),
        .p1(p1),
        .p1done(p1done),
        .p2(p2),
        .p2done(p2done)
    );

    int cycles;

    initial begin
        cycles = 0;
        clk = 1'b0;
        #10;
        while (~p2done) begin
            clk = ~clk;
            # 5;
            cycles += 1;
        end
        cycles /= 2;
        $display("max score: %-d", p2);
        $display("cycles: %-d", cycles);
        $finish();
    end

    initial begin
        rst = 1'b1;
        # 5;
        rst = 1'b0;
    end
endmodule
