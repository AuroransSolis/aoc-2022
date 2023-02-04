/*
DATA_BITS: Bit width of the data stored by the ROM.
ADDR_BITS: Bit width of the ROM address.
HEAD_BITS: Number of bits representing how many readers there can be.
INIT_FILE: File to read memory from. Should be a <???> file.
*/
module ManyHeadRom #(
    parameter int DATA_BITS,
    parameter int ADDR_BITS,
    parameter int MAX_ADDR,
    parameter int HEADS,
    parameter INIT_FILE
) (
    input logic [ADDR_BITS - 1:0] addrs [HEADS - 1:0],
    output logic [DATA_BITS - 1:0] outs [HEADS - 1:0]
);
    // localparam int MAX_ADDR = {ADDR_BITS{1'b1}};

    // initialise ROM somehow???
    logic [DATA_BITS - 1:0] data [MAX_ADDR - 1:0];

    initial begin
        $readmemh(INIT_FILE, data);
    end

    genvar i;
    generate
        for(i = 0; i < HEADS; i += 1) begin: mhr_gen_output_assigns
            assign outs[i] = addrs[i] < MAX_ADDR ? data[addrs[i]] : {DATA_BITS{1'b0}};
        end
    endgenerate
endmodule
