module TreeMax #(parameter int BITS, parameter int LEN) (
    input logic [BITS - 1:0] inputs [LEN - 1:0],
    output logic [BITS - 1:0] max
);
    generate
        if (LEN == 1) begin: maxfinder_len_1
            assign max = inputs[0];
        end else begin: maxfinder_len_g1
            logic [BITS - 1:0] l;
            logic [BITS - 1:0] r;
            if (LEN == 2) begin: maxfinder_len_2_lr
                assign '{l, r} = inputs;
            end else begin: maxfinder_len_g2_lr
                localparam int RHALF_LEN = LEN / 2;
                TreeMax #(.BITS(BITS), .LEN(RHALF_LEN)) mfr (
                    .inputs(inputs[0 +: RHALF_LEN]),
                    .max(r)
                );

                localparam int LHALF_LEN = LEN - RHALF_LEN;
                TreeMax #(.BITS(BITS), .LEN(LHALF_LEN)) mfl (
                    .inputs(inputs[RHALF_LEN +: LHALF_LEN]),
                    .max(l)
                );
            end
            assign max = l > r ? l : r;
        end
    endgenerate
endmodule

`define MFSTB 8
`define MFSTL 16
module TreeMaxSynthTest (
    input logic [`MFSTB * `MFSTL - 1:0] i,
    output logic [`MFSTB - 1:0] max
);
    TreeMax #(.BITS(`MFSTB), .LEN(`MFSTL)) mf (.inputs(i), .max(max));
endmodule
