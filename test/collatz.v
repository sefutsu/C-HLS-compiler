module collatz (
	input ap_clk,
	input ap_rst,
	input ap_start,
	output reg ap_done,
	output ap_idle,
	output reg ap_ready,
	input [31:0] ap_n,
	output reg [31:0] ap_return
);

reg [31:0] n;
reg [31:0] m;

reg [3:0] ap_fsm;
assign ap_idle = ap_fsm == 0;

always @(posedge ap_clk) begin
	if(ap_rst) begin
		n <= 0;
		m <= 0;
		ap_done <= 0;
		ap_ready <= 1;
		ap_return <= 0;
		ap_fsm <= 0;
	end else begin
		case(ap_fsm)
			0: begin
				if(ap_start) begin
					n <= ap_n;
					m <= 0;
					ap_ready <= 0;
					ap_done <= 0;
					ap_fsm <= 1;
				end
			end
			1: begin
				if((1) < (n)) begin
					ap_fsm <= 2;
				end else begin
					ap_fsm <= 10;
				end
			end
			2: begin
				if((n) & (1)) begin
					ap_fsm <= 3;
				end else begin
					ap_fsm <= 5;
				end
			end
			3: begin
				n <= ((n) * (3)) + (1);
				ap_fsm <= 4;
			end
			4: begin
				ap_fsm <= 6;
			end
			5: begin
				n <= (n) / (2);
				ap_fsm <= 6;
			end
			6: begin
				if((m) < (n)) begin
					ap_fsm <= 7;
				end else begin
					ap_fsm <= 9;
				end
			end
			7: begin
				m <= n;
				ap_fsm <= 8;
			end
			8: begin
				ap_fsm <= 9;
			end
			9: begin
				ap_fsm <= 1;
			end
			10: begin
				ap_return <= m;
				ap_ready <= 1;
				ap_done <= 1;
				ap_fsm <= 0;
			end
			default: begin
				ap_fsm <= 0;
			end
		endcase
	end
end
endmodule
