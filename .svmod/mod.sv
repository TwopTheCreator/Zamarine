module ZamarineComponents #(parameter MAX_COMPONENTS = 8)(
    input logic clk,
    input logic reset,
    input logic [2:0] component_id,   // ID of the component to control
    input logic activate,              // 1 = activate, 0 = deactivate
    output logic [MAX_COMPONENTS-1:0] status  // Status of all components
);

    // Internal register to track component statuses
    logic [MAX_COMPONENTS-1:0] components;

    // Status output
    assign status = components;

    // Component control logic
    always_ff @(posedge clk or posedge reset) begin
        if (reset) begin
            components <= '0;  // Clear all components on reset
        end else begin
            if (component_id < MAX_COMPONENTS) begin
                components[component_id] <= activate;
            end
        end
    end

endmodule
