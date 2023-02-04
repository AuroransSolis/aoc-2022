interface TrailingZerosInterface #(parameter int BITS);
    logic [BITS - 1:0] input_num;
    logic [$clog2(BITS + 1) - 1:0] trailing_zeros;

    modport CounterPorts (input input_num, output trailing_zeros);
endinterface

module EncodeTrailingZeros (input bit [1:0] d, output logic [1:0] q);
    always_comb begin
        case (d)
            2'b00: q = 2'b10;
            2'b10: q = 2'b01;
            default: q = 2'b00;
        endcase
    end
endmodule

// BITS must be a power of 2.
module ReduceTrailingZerosSimple #(parameter int BITS) (
    input logic [BITS - 1:0] i,
    output logic [$clog2(BITS + 1) - 1:0] o
);
    generate
        if (BITS == 2) begin: rtzs_encode_2
            always_comb begin
                case (i)
                    2'b00: o = 2'b10;
                    2'b10: o = 2'b01;
                    default: o = 2'b00;
                endcase
            end
        end else begin: rtzs_reduce
            localparam int OLEN = $clog2(BITS + 1);
            localparam int PREV_OLEN = $clog2(BITS);
            logic [PREV_OLEN - 1:0] lhalf_o;
            ReduceTrailingZerosSimple #(.BITS(BITS / 2)) rtzs_left (
                .i(i[BITS - 1:BITS / 2]),
                .o(lhalf_o)
            );

            logic [PREV_OLEN - 1:0] rhalf_o;
            ReduceTrailingZerosSimple #(.BITS(BITS / 2)) rtzs_right (
                .i(i[BITS / 2 - 1:0]),
                .o(rhalf_o)
            );

            always_comb begin
                if (rhalf_o[PREV_OLEN - 1] == 1'b0) begin
                    o[OLEN - 1] = 1'b0;
                    o[OLEN - 2:0] = rhalf_o;
                end else begin
                    o[OLEN - 1] = lhalf_o[OLEN - 2] & rhalf_o[OLEN - 2];
                    o[OLEN - 2] = ~lhalf_o[OLEN - 2];
                    o[OLEN - 3:0] = lhalf_o[OLEN - 3:0];
                end
            end
        end
    endgenerate
endmodule

module ReduceTrailingZerosHard #(parameter int BITS) (
    input logic [BITS - 1:0] i,
    output logic [$clog2(BITS + 1):0] o
);
    generate
        if (BITS == 1) begin: rtzh_encode_1
            assign o = i ? 2'b00 : 2'b01;
        end else begin: rtzh_recurse
            localparam int RHALF_BITS = 1 << ($clog2(BITS + 1) - 1);
            logic [$clog2(BITS + 1) - 1:0] rhalf_o;
            ReduceTrailingZerosSimple #(.BITS(RHALF_BITS)) rhalf_rtzs (
                .i(i[0 +: RHALF_BITS]),
                .o(rhalf_o)
            );

            logic [$clog2(BITS + 1) - 1:0] lhalf_o;

            localparam int LHALF_BITS = BITS - RHALF_BITS;
            localparam int LHALF_BITS_PO2 = LHALF_BITS == 1 << ($clog2(LHALF_BITS + 1) - 1);
            localparam int LHALF_BITS_EXT = LHALF_BITS % 2;
            localparam int LHALF_OLEN = $clog2(LHALF_BITS + 1) + LHALF_BITS_EXT;
            if (LHALF_OLEN < $clog2(BITS + 1)) begin: rtzh_assign_lhalf_leftover
                assign lhalf_o[$clog2(BITS + 1) - 1:LHALF_OLEN] = {
                    {$clog2(BITS + 1) - LHALF_OLEN}{1'b0}
                };
            end
            if (LHALF_BITS_PO2 && LHALF_BITS > 1) begin: rtzh_lhalf_rtzs
                ReduceTrailingZerosSimple #(.BITS(LHALF_BITS)) lhalf_rtzs (
                    .i(i[RHALF_BITS +: LHALF_BITS]),
                    .o(lhalf_o[0 +: LHALF_OLEN])
                );
            end else begin: rtzh_lhalf_rtzh
                ReduceTrailingZerosHard #(.BITS(LHALF_BITS)) lhalf_rtzh (
                    .i(i[RHALF_BITS +: LHALF_BITS]),
                    .o(lhalf_o[0 +: LHALF_OLEN])
                );
            end

            localparam int OLEN = $clog2(BITS + 1);

            always_comb begin
                if(rhalf_o[OLEN - 1] == 1'b0) begin
                    o[OLEN] = rhalf_o[OLEN - 1] & lhalf_o[OLEN - 1];
                    o[OLEN - 1] = 1'b0;
                    o[OLEN - 2:0] = rhalf_o[OLEN - 2:0];
                end else begin
                    o[BITS] = rhalf_o[OLEN - 1] & lhalf_o[OLEN - 1];
                    o[OLEN - 1] = ~lhalf_o[OLEN - 1];
                    o[OLEN - 2:0] = lhalf_o[OLEN - 2:0];
                end
            end
        end
    endgenerate
endmodule

module TrailingZeros #(parameter int BITS) (
    TrailingZerosInterface.CounterPorts tzi
);
    generate
        if (BITS == 1) begin: tz_len_1
            assign tzi.trailing_zeros = ~tzi.input_num;
        end else begin: tz_len_g1
            localparam bit IS_PO2 = BITS == 1 << ($clog2(BITS + 1) - 1);
            if (IS_PO2 && BITS > 1) begin: tz_simple_case
                ReduceTrailingZerosSimple #(.BITS(BITS)) rtzs (
                    .i(tzi.input_num),
                    .o(tzi.trailing_zeros)
                );
            end else begin: tz_hard_case
                logic [$clog2(BITS + 1):0] out_ext;
                ReduceTrailingZerosHard #(.BITS(BITS)) reduce (
                    .i(tzi.input_num),
                    .o(out_ext)
                );
                assign tzi.trailing_zeros = out_ext[0 +: $clog2(BITS + 1)];
            end
        end
    endgenerate
endmodule

`timescale 1ns/1ns
module TrailingZerosTest ();
    localparam BITS = 5;

    logic [BITS - 1:0] data;
    logic [$clog2(BITS + 1) - 1:0] tz;

    TrailingZerosInterface #(.BITS(BITS)) tzi ();
    assign tzi.input_num = data;
    assign tz = tzi.trailing_zeros;

    TrailingZeros #(.BITS(BITS)) tzc (tzi.CounterPorts);

    initial begin
        data = ~{BITS{1'b0}};
        while (data != {BITS{1'b0}}) begin
            # 10
            data -= 1;
        end
    end
endmodule

`define TZST 128
module TrailingZerosSynthTest (
    input logic [`TZST - 1:0] i,
    output logic [$clog2(`TZST + 1) - 1:0] o
);
    TrailingZerosInterface #(.BITS(`TZST)) tzi ();
    assign tzi.input_num = i;
    assign o = tzi.trailing_zeros;

    TrailingZeros #(.BITS(`TZST)) tz (tzi.CounterPorts);
endmodule
