-- this was hell to make
-- fucking shit for brains twop told me to learn lua
-- i did
-- and here it is fuckers
local config = {
    rcp1 = {
        port = 8080,
        max_connections = 100,
        connection_timeout = 30000,
        log_level = "INFO"
    },
    security = {
        enable_encryption = true,
        key_file = "/etc/zamarine/rcp1/keys/rcp1.key",
        allowed_ciphers = {"AES-256-GCM", "CHACHA20-POLY1305"}
    },
    performance = {
        thread_pool_size = 10,
        buffer_size = 8192,
        max_message_size = 1048576,
        compression = {
            enabled = true,
            algorithm = "zstd",
            level = 3
        }
    },
    monitoring = {
        enabled = true,
        metrics_port = 9090,
        collect_interval = 60
    }
}

-- Runtime validation
local function validate_config()
    if config.rcp1.port < 1024 or config.rcp1.port > 65535 then
        error("Invalid port number: " .. tostring(config.rcp1.port))
    end
    
    if config.performance.thread_pool_size < 1 then
        config.performance.thread_pool_size = 1
    end
    
    return true
end

-- Initialize configuration
validate_config()

return config
