interface Day8Part2SyncInterface #(
    parameter int ADDR_BITS,
    parameter bit [ADDR_BITS - 1:0] MAX_C,
    parameter bit [ADDR_BITS - 1:0] MAX_R,
    parameter int CHECKERS
) (input bit clk);
    logic rst;
    logic ind_cell_locked;
    logic [$clog2(CHECKERS + 1) - 1:0] ind_cell_locked_to;
    logic [ADDR_BITS - 1:0] from_ind_cell;
    logic [CHECKERS - 1:0] ind_cell_lock_reqs;
    logic [ADDR_BITS - 1:0] to_ind_cells [CHECKERS - 1:0];
    logic [ADDR_BITS - 1:0] mhr_cs_addr [CHECKERS - 1:0];
    logic [7:0] mhr_cs_out [CHECKERS - 1:0];
    logic [ADDR_BITS - 1:0] mhr_us_addr [CHECKERS - 1:0];
    logic [7:0] mhr_us_out [CHECKERS - 1:0];
    logic [ADDR_BITS - 1:0] mhr_rs_addr [CHECKERS - 1:0];
    logic [7:0] mhr_rs_out [CHECKERS - 1:0];
    logic [ADDR_BITS - 1:0] mhr_ds_addr [CHECKERS - 1:0];
    logic [7:0] mhr_ds_out [CHECKERS - 1:0];
    logic [ADDR_BITS - 1:0] mhr_ls_addr [CHECKERS - 1:0];
    logic [7:0] mhr_ls_out [CHECKERS - 1:0];
    logic [17:0] p2sol;
    logic completed;

    modport D8P2SPorts (
        input clk,
        input rst,
        input ind_cell_locked,
        input ind_cell_locked_to,
        input from_ind_cell,
        input mhr_cs_out,
        input mhr_us_out,
        input mhr_rs_out,
        input mhr_ds_out,
        input mhr_ls_out,
        output ind_cell_lock_reqs,
        output to_ind_cells,
        output mhr_cs_addr,
        output mhr_us_addr,
        output mhr_rs_addr,
        output mhr_ds_addr,
        output mhr_ls_addr,
        output p2sol,
        output completed
    );
endinterface

module Day8Part2Sync #(
    parameter int ADDR_BITS,
    parameter int MAX_C,
    parameter int MAX_R,
    parameter int CHECKERS
) (
    Day8Part2SyncInterface.D8P2SPorts si
);
    logic [17:0] checker_maxes [CHECKERS - 1:0];
    logic [CHECKERS - 1:0] completed_checkers;

    genvar i;
    generate
        for(i = 0; i < CHECKERS; i += 1) begin: d8p2s_gen_checkers
            Day8Part2SyncCheckerInterface #(
                .ADDR_BITS(ADDR_BITS),
                .MAX_C(MAX_C),
                .MAX_R(MAX_R),
                .ACTORS(CHECKERS)
            ) d8p2sci (si.clk);
            assign d8p2sci.rst = si.rst;
            assign d8p2sci.ind_cell_locked = si.ind_cell_locked;
            assign d8p2sci.ind_cell_locked_to = si.ind_cell_locked_to;
            assign d8p2sci.from_ind_cell = si.from_ind_cell;
            assign si.ind_cell_lock_reqs[i] = d8p2sci.ind_cell_lock_req;
            assign si.to_ind_cells[i] = d8p2sci.to_ind_cell;
            assign si.mhr_cs_addr[i] = d8p2sci.mhr_c_addr;
            assign d8p2sci.mhr_c_out = si.mhr_cs_out[i];
            assign si.mhr_us_addr[i] = d8p2sci.mhr_u_addr;
            assign d8p2sci.mhr_u_out = si.mhr_us_out[i];
            assign si.mhr_rs_addr[i] = d8p2sci.mhr_r_addr;
            assign d8p2sci.mhr_r_out = si.mhr_rs_out[i];
            assign si.mhr_ds_addr[i] = d8p2sci.mhr_d_addr;
            assign d8p2sci.mhr_d_out = si.mhr_ds_out[i];
            assign si.mhr_ls_addr[i] = d8p2sci.mhr_l_addr;
            assign d8p2sci.mhr_l_out = si.mhr_ls_out[i];
            assign checker_maxes[i] = d8p2sci.max_score;
            assign completed_checkers[i] = d8p2sci.completed;

            Day8Part2SyncChecker #(
                .ADDR_BITS(ADDR_BITS),
                .MAX_C(MAX_C),
                .MAX_R(MAX_R),
                .CHECKERS(CHECKERS),
                .CHECKER_ID(i)
            ) d8p2_checker (
                .sci(d8p2sci.D8P2SCPorts)
            );
        end
    endgenerate

    logic [17:0] current_max;
    TreeMax #(.BITS(18), .LEN(CHECKERS)) mf (.inputs(checker_maxes), .max(current_max));

    assign si.completed = completed_checkers == {CHECKERS{1'b1}};
    assign si.p2sol = si.completed ? current_max : 18'b0;
endmodule

package SyncCheckerState;
    typedef enum {
        GetIndLock,
        AwaitIndLock,
        UpdateInd,
        ReleaseIndLock,
        CountTrees,
        UpdateMax,
        Complete
    } SyncCheckerState;
endpackage

interface Day8Part2SyncCheckerInterface #(
    parameter int ADDR_BITS,
    parameter bit [ADDR_BITS - 1:0] MAX_C,
    parameter bit [ADDR_BITS - 1:0] MAX_R,
    parameter int ACTORS
) (input bit clk);
    logic rst;
    logic ind_cell_locked;
    logic [$clog2(ACTORS + 1) - 1:0] ind_cell_locked_to;
    logic [ADDR_BITS - 1:0] from_ind_cell;
    logic ind_cell_lock_req;
    logic [ADDR_BITS - 1:0] to_ind_cell;
    logic [ADDR_BITS - 1:0] mhr_c_addr;
    logic [7:0] mhr_c_out;
    logic [ADDR_BITS - 1:0] mhr_u_addr;
    logic [7:0] mhr_u_out;
    logic [ADDR_BITS - 1:0] mhr_r_addr;
    logic [7:0] mhr_r_out;
    logic [ADDR_BITS - 1:0] mhr_d_addr;
    logic [7:0] mhr_d_out;
    logic [ADDR_BITS - 1:0] mhr_l_addr;
    logic [7:0] mhr_l_out;

    logic [17:0] max_score;
    logic completed;

    modport D8P2SCPorts (
        input rst,
        input clk,
        input ind_cell_locked,
        input ind_cell_locked_to,
        input from_ind_cell,
        input mhr_c_out,
        input mhr_u_out,
        input mhr_r_out,
        input mhr_d_out,
        input mhr_l_out,
        output ind_cell_lock_req,
        output to_ind_cell,
        output mhr_c_addr,
        output mhr_u_addr,
        output mhr_r_addr,
        output mhr_d_addr,
        output mhr_l_addr,
        output max_score,
        output completed
    );
endinterface

module Day8Part2SyncChecker #(
    parameter int ADDR_BITS,
    parameter bit [ADDR_BITS - 1:0] MAX_C,
    parameter bit [ADDR_BITS - 1:0] MAX_R,
    parameter int CHECKERS,
    parameter bit [$clog2(CHECKERS + 1) - 1:0] CHECKER_ID
) (
    Day8Part2SyncCheckerInterface.D8P2SCPorts sci
);
    localparam int MAXIND = MAX_R * (MAX_C + 1);
    localparam bit [ADDR_BITS - 1:0] NEXTCHAR_OFFSET = {{{ADDR_BITS - 1}{1'b0}}, 1'b1};
    localparam bit [ADDR_BITS - 1:0] NEXTLINE_OFFSET = {{{ADDR_BITS - 3}{1'b0}}, 3'b100};
    localparam int PADDED_W = MAX_C + 1;
    localparam bit [ADDR_BITS - 1:0] UD_OFFSET = PADDED_W[ADDR_BITS - 1:0];
    localparam bit [ADDR_BITS - 1:0] LR_OFFSET = {{{ADDR_BITS - 1}{1'b0}}, 1'b1};

    SyncCheckerState::SyncCheckerState checker_state;
    assign sci.completed = checker_state == SyncCheckerState::Complete;

    logic u_done;
    logic r_done;
    logic d_done;
    logic l_done;
    logic all_done;
    assign all_done = u_done & r_done & d_done & l_done;

    logic [17:0] u_count;
    logic [17:0] r_count;
    logic [17:0] d_count;
    logic [17:0] l_count;
    logic [17:0] score;
    assign score = u_count * r_count * d_count * l_count;

    logic [ADDR_BITS - 1:0] ind;

    always_ff @ (posedge sci.clk or posedge sci.rst) begin
        if (sci.rst) begin
            checker_state <= SyncCheckerState::GetIndLock;
            u_done <= 1'b0;
            r_done <= 1'b0;
            d_done <= 1'b0;
            l_done <= 1'b0;
            u_count <= 18'b0;
            r_count <= 18'b0;
            d_count <= 18'b0;
            l_count <= 18'b0;
            sci.ind_cell_lock_req <= 1'b0;
            sci.to_ind_cell <= {ADDR_BITS{1'b0}};
            sci.max_score <= 1'b0;
        end else if (checker_state != SyncCheckerState::Complete) begin
            if (checker_state == SyncCheckerState::GetIndLock) begin
                sci.ind_cell_lock_req <= 1'b1;
                checker_state <= SyncCheckerState::AwaitIndLock;
            end else if (checker_state == SyncCheckerState::AwaitIndLock) begin
                if (sci.ind_cell_locked && sci.ind_cell_locked_to == CHECKER_ID) begin
                    checker_state <= SyncCheckerState::UpdateInd;
                    ind <= sci.from_ind_cell;
                    sci.mhr_r_addr <= sci.from_ind_cell + {{{ADDR_BITS - 2}{1'b0}}, 2'b10};
                end
            end else if (checker_state == SyncCheckerState::UpdateInd) begin
                if (ind > MAXIND - UD_OFFSET || ind < UD_OFFSET) begin
                    sci.ind_cell_lock_req <= 1'b0;
                    checker_state <= SyncCheckerState::Complete;
                end else begin
                    if (sci.mhr_r_out == 8'd10) begin
                        sci.to_ind_cell <= ind + NEXTLINE_OFFSET;
                    end else begin
                        sci.to_ind_cell <= ind + NEXTCHAR_OFFSET;
                    end
                    checker_state <= SyncCheckerState::ReleaseIndLock;
                end
            end else if (checker_state == SyncCheckerState::ReleaseIndLock) begin
                sci.ind_cell_lock_req <= 1'b0;
                sci.mhr_c_addr <= ind;
                sci.mhr_u_addr <= ind - UD_OFFSET;
                sci.mhr_r_addr <= ind + LR_OFFSET;
                sci.mhr_d_addr <= ind + UD_OFFSET;
                sci.mhr_l_addr <= ind - LR_OFFSET;
                u_done <= 1'b0;
                r_done <= 1'b0;
                d_done <= 1'b0;
                l_done <= 1'b0;
                u_count <= 18'b1;
                r_count <= 18'b0;
                d_count <= 18'b1;
                l_count <= 18'b0;
                checker_state <= SyncCheckerState::CountTrees;
            end else if (checker_state == SyncCheckerState::CountTrees) begin
                if (all_done) begin
                    checker_state <= SyncCheckerState::UpdateMax;
                end else begin
                    if (!u_done) begin
                        if (sci.mhr_u_addr < UD_OFFSET) begin
                            u_done <= 1'b1;
                        end else begin
                            if (sci.mhr_u_out >= sci.mhr_c_out) begin
                                u_done <= 1'b1;
                            end else begin
                                sci.mhr_u_addr <= sci.mhr_u_addr - UD_OFFSET;
                                u_count <= u_count + 18'b1;
                            end
                        end
                    end
                    if (!r_done) begin
                        if (sci.mhr_r_out == 8'd10) begin
                            r_done <= 1'b1;
                        end else begin
                            r_count <= r_count + 18'b1;
                            if (sci.mhr_r_out >= sci.mhr_c_out) begin
                                r_done <= 1'b1;
                            end else begin
                                sci.mhr_r_addr <= sci.mhr_r_addr + LR_OFFSET;
                            end
                        end
                    end
                    if (!d_done) begin
                        if (sci.mhr_d_addr > MAXIND - UD_OFFSET) begin
                            d_done <= 1'b1;
                        end else begin
                            if (sci.mhr_d_out >= sci.mhr_c_out) begin
                                d_done <= 1'b1;
                            end else begin
                                sci.mhr_d_addr <= sci.mhr_d_addr + UD_OFFSET;
                                d_count <= d_count + 18'b1;
                            end
                        end
                    end
                    if (!l_done) begin
                        if (sci.mhr_l_out == 8'd10) begin
                            l_done <= 1'b1;
                        end else begin
                            l_count <= l_count + 18'b1;
                            if (sci.mhr_l_out >= sci.mhr_c_out) begin
                                l_done <= 1'b1;
                            end else begin
                                sci.mhr_l_addr <= sci.mhr_l_addr - LR_OFFSET;
                            end
                        end
                    end
                end
            end else if (checker_state == SyncCheckerState::UpdateMax) begin
                if (score > sci.max_score) begin
                    sci.max_score <= score;
                end
                checker_state <= SyncCheckerState::GetIndLock;
            end
        end
    end
endmodule
