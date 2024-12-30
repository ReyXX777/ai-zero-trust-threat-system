-- Create the database schema for the AI-Driven Zero Trust Threat Intelligence System

-- Create table to store information about threats detected
CREATE TABLE threats (
    id SERIAL PRIMARY KEY,
    threat_type VARCHAR(255) NOT NULL,
    severity VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create table to store behavioral analysis results
CREATE TABLE behavioral_analysis (
    id SERIAL PRIMARY KEY,
    analysis_result FLOAT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create table for storing logs related to security events
CREATE TABLE security_logs (
    id SERIAL PRIMARY KEY,
    log_level VARCHAR(50) NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create table for storing Kafka topics or any relevant Kafka configurations
CREATE TABLE kafka_config (
    id SERIAL PRIMARY KEY,
    broker VARCHAR(255) NOT NULL,
    topic VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Optional: Create table for storing user-specific settings (if needed for user authentication or analysis)
CREATE TABLE user_settings (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    setting_key VARCHAR(255) NOT NULL,
    setting_value TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Optional: Insert default Kafka broker configuration if needed
INSERT INTO kafka_config (broker, topic) VALUES
    ('localhost:9092', 'threat_detection'),
    ('localhost:9092', 'behavioral_analysis');

-- Optional: Insert some default system settings or test data
INSERT INTO threats (threat_type, severity, description) VALUES
    ('malware', 'high', 'Critical malware detected in the system'),
    ('phishing', 'medium', 'Phishing attempt detected via email');

-- Insert some default log entries for system monitoring
INSERT INTO security_logs (log_level, message) VALUES
    ('INFO', 'System initialized successfully'),
    ('ERROR', 'Failed to connect to Kafka broker');

-- Optionally create indexes to optimize queries
CREATE INDEX idx_threats_created_at ON threats(created_at);
CREATE INDEX idx_behavioral_analysis_created_at ON behavioral_analysis(created_at);
CREATE INDEX idx_security_logs_created_at ON security_logs(created_at);
