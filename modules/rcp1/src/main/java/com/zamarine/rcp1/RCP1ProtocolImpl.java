package com.zamarine.rcp1;

import java.io.*;
import java.net.*;
import java.util.Properties;
import org.luaj.vm2.*;
import org.luaj.vm2.lib.jse.*;

public class RCP1ProtocolImpl implements RCP1Protocol {
    private static final String CONFIG_FILE = "/etc/zamarine/rcp1/config.ini";
    private Properties config;
    private LuaValue luaRuntime;
    private ServerSocket serverSocket;
    private boolean isRunning;

    public RCP1ProtocolImpl() {
        this.config = new Properties();
        this.luaRuntime = JsePlatform.standardGlobals();
        this.isRunning = false;
    }

    @Override
    public void initialize() throws RCP1Exception {
        try {
            loadLuaConfig("lua/rcp1_init.lua");
            serverSocket = new ServerSocket(Integer.parseInt(
                config.getProperty("rcp1.port", "8080")));
            isRunning = true;
        } catch (Exception e) {
            throw new RCP1Exception("Failed to initialize RCP1 protocol", e);
        }
    }

    @Override
    public void send(byte[] data, String destination) throws RCP1Exception {
        try (Socket socket = new Socket(destination, 
                Integer.parseInt(config.getProperty("rcp1.port", "8080")));
             DataOutputStream out = new DataOutputStream(socket.getOutputStream())) {
            out.writeInt(data.length);
            out.write(data);
        } catch (IOException e) {
            throw new RCP1Exception("Failed to send RCP1 data", e);
        }
    }

    @Override
    public byte[] receive() throws RCP1Exception {
        try (Socket clientSocket = serverSocket.accept();
             DataInputStream in = new DataInputStream(clientSocket.getInputStream())) {
            int length = in.readInt();
            byte[] data = new byte[length];
            in.readFully(data);
            return data;
        } catch (IOException e) {
            throw new RCP1Exception("Failed to receive RCP1 data", e);
        }
    }

    @Override
    public void configure(String configPath) throws RCP1Exception {
        try (FileInputStream fis = new FileInputStream(
                configPath != null ? configPath : CONFIG_FILE)) {
            config.load(fis);
            loadLuaConfig(config.getProperty("rcp1.lua_config", "lua/rcp1_config.lua"));
        } catch (IOException e) {
            throw new RCP1Exception("Failed to load RCP1 configuration", e);
        }
    }

    @Override
    public void shutdown() throws RCP1Exception {
        isRunning = false;
        try {
            if (serverSocket != null && !serverSocket.isClosed()) {
                serverSocket.close();
            }
        } catch (IOException e) {
            throw new RCP1Exception("Error during RCP1 shutdown", e);
        }
    }

    private void loadLuaConfig(String luaPath) throws RCP1Exception {
        try {
            LuaValue chunk = luaRuntime.get("dofile").call(LuaValue.valueOf(luaPath));
            if (chunk.istable()) {
                LuaValue configTable = chunk.checktable();
                for (LuaValue key : configTable.keys()) {
                    config.setProperty(key.tojstring(), configTable.get(key).tojstring());
                }
            }
        } catch (Exception e) {
            throw new RCP1Exception("Failed to load Lua configuration: " + luaPath, e);
        }
    }
}
