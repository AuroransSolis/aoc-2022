interface SyncCellActor #(parameter int ACTORS, parameter int DATA_BITS);
    logic lock_req;
    logic [DATA_BITS - 1:0] to_cell;
    logic is_locked;
    logic [$clog2(ACTORS + 1) - 1:0] locked_to;
    logic [DATA_BITS - 1:0] from_cell;

    modport SyncCellActorPorts (
        input is_locked,
        input locked_to,
        input from_cell,
        output lock_req,
        output to_cell
    );
endinterface

interface SyncCellInterface #(parameter int ACTORS, parameter int DATA_BITS) (input bit clk);
    logic rst;
    logic [ACTORS - 1:0] lock_reqs;
    logic [DATA_BITS - 1:0] data_inputs [ACTORS - 1:0];
    logic is_locked;
    logic [$clog2(ACTORS + 1) - 1:0] locked_to;
    logic [DATA_BITS - 1:0] data_out;

    modport SyncCellPorts (
        input clk,
        input rst,
        input lock_reqs,
        input data_inputs,
        output is_locked,
        output locked_to,
        output data_out
    );
endinterface

module SyncCell #(
    parameter int ACTORS,
    parameter int DATA_BITS,
    parameter bit [DATA_BITS - 1:0] RST_VAL
) (
    SyncCellInterface.SyncCellPorts sci
);
    localparam bit [DATA_BITS - 1:0] DATA_ZERO = {DATA_BITS{1'b0}};

    TrailingZerosInterface #(.BITS(ACTORS)) tzi ();
    assign tzi.input_num = sci.lock_reqs;
    TrailingZeros #(.BITS(ACTORS)) trailing_zero_counter (.tzi(tzi));

    logic [DATA_BITS - 1:0] new_data;
    always_comb begin
        new_data = sci.data_inputs[sci.locked_to];
    end

    always_ff @ (posedge sci.rst or negedge sci.clk) begin
        if (sci.rst) begin
            sci.is_locked <= 1'b0;
            sci.locked_to <= {{$clog2(ACTORS + 1)}{1'b0}};
            sci.data_out <= RST_VAL;
        end else begin
            if (sci.is_locked) begin
                // If we're locked, check that the lock is still being requested.
                if (!sci.lock_reqs[sci.locked_to]) begin
                    // If it isn't, release the lock.
                    sci.is_locked <= 1'b0;
                end else begin
                    sci.data_out <= new_data;
                end
            end else if(|sci.lock_reqs) begin
                // Otherwise, give the lock to the requester with the smallest ID using a
                // trailing zero counter.
                sci.locked_to <= tzi.trailing_zeros;
                sci.is_locked <= 1'b1;
            end
        end
    end
endmodule

module SyncCellTest();
    // test this somehow i dunno a good way to do it right now at 0159 on a tuesday morning
endmodule

`define SCSTA 16
`define SCSTD 8
module SyncCellSynthTest (
    input bit clk,
    input bit rst,
    input bit [`SCSTA - 1:0] lock_reqs,
    input bit [`SCSTA * `SCSTD - 1:0] data_inputs,
    output logic is_locked,
    output logic [$clog2(`SCSTA + 1) - 1:0] locked_to,
    output logic [`SCSTD - 1:0] data_out
);
    SyncCellInterface #(.ACTORS(`SCSTA), .DATA_BITS(`SCSTD)) sci (clk);
    assign sci.rst = rst;
    assign sci.lock_reqs = lock_reqs;
    assign sci.data_inputs = data_inputs;
    assign is_locked = sci.is_locked;
    assign locked_to = sci.locked_to;
    assign data_out = sci.data_out;

    SyncCell #(.ACTORS(`SCSTA), .DATA_BITS(`SCSTD), .RST_VAL({`SCSTD{1'b0}})) sc (.sci(sci));
endmodule
