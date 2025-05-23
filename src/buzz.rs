use anyhow::Result;
use rand::seq::SliceRandom;
use serenity::all::Message;
use tracing::error;

pub struct BuzzGenerator;

impl BuzzGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_buzzword(&self) -> String {
        let mut rng = rand::thread_rng();
        
        // First part of the buzzword phrase - adjectives
        let first_words = [
            "Adaptive", "Advanced", "Ameliorated", "Assimilated", "Automated", "Balanced",
            "Business-focused", "Centralized", "Cloned", "Compatible", "Configurable", 
            "Cross-group", "Cross-platform", "Customer-focused", "Customizable", "Decentralized",
            "De-engineered", "Devolved", "Digitized", "Distributed", "Diverse", "Down-sized",
            "Enhanced", "Enterprise-wide", "Ergonomic", "Exclusive", "Expanded", "Extended",
            "Face to face", "Focused", "Front-line", "Fully-configurable", "Function-based", 
            "Fundamental", "Future-proofed", "Grass-roots", "Horizontal", "Implemented",
            "Innovative", "Integrated", "Intuitive", "Inverse", "Managed", "Mandatory",
            "Monitored", "Multi-channelled", "Multi-lateral", "Multi-layered", "Multi-tiered", 
            "Networked", "Object-based", "Open-architected", "Open-source", "Operative",
            "Optimized", "Optional", "Organic", "Organized", "Persevering", "Persistent",
            "Phased", "Polarised", "Pre-emptive", "Proactive", "Profit-focused", "Profound",
            "Programmable", "Progressive", "Public-key", "Quality-focused", "Reactive", 
            "Realigned", "Re-contextualized", "Re-engineered", "Reduced", "Reverse-engineered",
            "Right-sized", "Robust", "Seamless", "Secured", "Self-enabling", "Sharable",
            "Stand-alone", "Streamlined", "Switchable", "Synchronised", "Synergistic", 
            "Synergized", "Team-oriented", "Total", "Triple-buffered", "Universal",
            "Up-sized", "Upgradable", "User-centric", "User-friendly", "Versatile", 
            "Virtual", "Visionary", "Vision-oriented",
        ];
        
        // Second part of the buzzword phrase - modifiers
        let second_words = [
            "24 hour", "24/7", "3rd generation", "4th generation", "5th generation", 
            "6th generation", "actuating", "analyzing", "assymetric", "asynchronous",
            "attitude-oriented", "background", "bandwidth-monitored", "bi-directional", 
            "bifurcated", "bottom-line", "clear-thinking", "client-driven",
            "client-server", "coherent", "cohesive", "composite", "context-sensitive", 
            "contextually-based", "content-based", "dedicated", "demand-driven", "didactic",
            "directional", "discrete", "disintermediate", "dynamic", "eco-centric", 
            "empowering", "encompassing", "even-keeled", "executive", "explicit",
            "exuding", "fault-tolerant", "foreground", "fresh-thinking", "full-range", 
            "global", "grid-enabled", "heuristic", "high-level", "holistic",
            "homogeneous", "human-resource", "hybrid", "impactful", "incremental", 
            "intangible", "interactive", "intermediate", "leading edge", "local",
            "logistical", "maximized", "methodical", "mission-critical", "mobile", 
            "modular", "motivating", "multimedia", "multi-state", "multi-tasking",
            "national", "needs-based", "neutral", "next generation", "non-volatile", 
            "object-oriented", "optimal", "optimizing", "radical", "real-time",
            "reciprocal", "regional", "responsive", "scalable", "secondary", "solution-oriented", 
            "stable", "static", "systematic", "systemic", "system-worthy", "tangible",
            "tertiary", "transitional", "uniform", "upward-trending", "user-facing", "value-added", 
            "web-enabled", "well-modulated", "zero administration", "zero defect", "zero tolerance",
        ];
        
        // Third part of the buzzword phrase - nouns
        let third_words = [
            "ability", "access", "adapter", "algorithm", "alliance", "analyzer", "application", 
            "approach", "architecture", "archive", "artificial intelligence", "array",
            "attitude", "benchmark", "budgetary management", "capability", "capacity", 
            "challenge", "circuit", "collaboration", "complexity", "concept",
            "conglomeration", "contingency", "core", "customer loyalty", "database", 
            "data-warehouse", "definition", "emulation", "encoding", "encryption",
            "extranet", "firmware", "flexibility", "focus group", "forecast", "frame", 
            "framework", "function", "functionalities", "Graphic Interface",
            "groupware", "Graphical User Interface", "hardware", "help-desk", "hierarchy", 
            "hub", "implementation", "info-mediaries", "infrastructure",
            "initiative", "installation", "instruction set", "interface", "internet solution", 
            "intranet", "knowledge user", "knowledge base", "local area network",
            "leverage", "matrices", "matrix", "methodology", "middleware", "migration", 
            "model", "moderator", "monitoring", "moratorium", "neural-net",
            "open architecture", "open system", "orchestration", "paradigm", "parallelism", 
            "policy", "portal", "pricing structure", "process improvement",
            "product", "productivity", "project", "projection", "protocol", "secured line", 
            "service-desk", "software", "solution", "standardization",
            "strategy", "structure", "success", "superstructure", "support", "synergy", 
            "system engine", "task-force", "throughput", "time-frame",
            "toolset", "utilisation", "website", "workforce",
        ];
        
        // Select one word from each array
        let first = first_words.choose(&mut rng).unwrap();
        let second = second_words.choose(&mut rng).unwrap();
        let third = third_words.choose(&mut rng).unwrap();
        
        // Combine them into a buzzword phrase
        format!("{} {} {}", first, second, third)
    }
}

pub async fn handle_buzz_command(http: &serenity::http::Http, msg: &Message) -> Result<()> {
    let generator = BuzzGenerator::new();
    let buzzword = generator.generate_buzzword();
    
    if let Err(e) = msg.channel_id.say(http, buzzword).await {
        error!("Error sending buzzword: {:?}", e);
    }
    
    Ok(())
}
