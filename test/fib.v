module fib (
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
reg [31:0] a;
reg [31:0] b;
reg [31:0] tmp;

reg [3:0] ap_fsm;
assign ap_idle = ap_fsm == 0;

always @(posedge ap_clk) begin
	if(ap_rst) begin
		n <= 0;
		a <= 0;
		b <= 1;
		tmp <= 0;
		ap_done <= 0;
		ap_ready <= 1;
		ap_return <= 0;
		ap_fsm <= 0;
	end else begin
		case(ap_fsm)
			0: begin
				if(ap_start) begin
					n <= ap_n;
					a <= 0;
					b <= 1;
					tmp <= 0;
					ap_ready <= 0;
					ap_done <= 0;
					ap_fsm <= 1;
				end
			end
			1: begin
				if(($signed(n)) < (2)) begin
					ap_fsm <= 2;
				end else begin
					ap_fsm <= 4;
				end
			end
			2: begin
				ap_return <= $signed(n);
				ap_ready <= 1;
				ap_done <= 1;
				ap_fsm <= 0;
			end
			3: begin
				ap_fsm <= 4;
			end
			4: begin
				if((1) < ($signed(n))) begin
					ap_fsm <= 5;
				end else begin
					ap_fsm <= 11;
				end
			end
			5: begin
				a <= ($signed(a)) + ($signed(b));
				ap_fsm <= 6;
			end
			6: begin
				tmp <= $signed(a);
				ap_fsm <= 7;
			end
			7: begin
				a <= $signed(b);
				ap_fsm <= 8;
			end
			8: begin
				b <= $signed(tmp);
				ap_fsm <= 9;
			end
			9: begin
				n <= ($signed(n)) - (1);
				ap_fsm <= 10;
			end
			10: begin
				ap_fsm <= 4;
			end
			11: begin
				ap_return <= $signed(b);
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
