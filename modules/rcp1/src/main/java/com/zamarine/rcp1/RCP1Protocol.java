package com.zamarine.rcp1;

public interface RCP1Protocol {
    void initialize() throws RCP1Exception;
    void send(byte[] data, String destination) throws RCP1Exception;
    byte[] receive() throws RCP1Exception;
    void configure(String configPath) throws RCP1Exception;
    void shutdown() throws RCP1Exception;
}
