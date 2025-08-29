package com.zamarine.rcp1;

import java.io.File;

public class RCP1Main {
    private static final String DEFAULT_CONFIG = "/etc/zamarine/rcp1/rcp1.ini";
    
    public static void main(String[] args) {
        try {
            String configPath = args.length > 0 ? args[0] : DEFAULT_CONFIG;
            
            System.out.println("Starting Zamarine RCP1 Module...");
            System.out.println("Configuration: " + new File(configPath).getAbsolutePath());
            
            RCP1Protocol rcp1 = new RCP1ProtocolImpl();
            rcp1.configure(configPath);
            rcp1.initialize();
            
            System.out.println("RCP1 Module started successfully.");
            
            // Add shutdown hook
            Runtime.getRuntime().addShutdownHook(new Thread(() -> {
                try {
                    System.out.println("\nShutting down RCP1 Module...");
                    rcp1.shutdown();
                    System.out.println("RCP1 Module stopped.");
                } catch (RCP1Exception e) {
                    System.err.println("Error during shutdown: " + e.getMessage());
                }
            }));
            
            // Keep the main thread alive
            while (true) {
                try {
                    Thread.sleep(1000);
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    break;
                }
            }
            
        } catch (Exception e) {
            System.err.println("Fatal error in RCP1 Module: " + e.getMessage());
            e.printStackTrace();
            System.exit(1);
        }
    }
}
