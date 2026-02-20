#![recursion_limit = "256"]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::assigning_clones,
    clippy::bool_to_int_with_if,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::cast_possible_wrap,
    clippy::doc_markdown,
    clippy::field_reassign_with_default,
    clippy::float_cmp,
    clippy::implicit_clone,
    clippy::items_after_statements,
    clippy::map_unwrap_or,
    clippy::manual_let_else,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::new_without_default,
    clippy::needless_pass_by_value,
    clippy::needless_raw_string_hashes,
    clippy::redundant_closure_for_method_calls,
    clippy::return_self_not_must_use,
    clippy::similar_names,
    clippy::single_match_else,
    clippy::struct_field_names,
    clippy::too_many_lines,
    clippy::uninlined_format_args,
    clippy::unnecessary_cast,
    clippy::unnecessary_lazy_evaluations,
    clippy::unnecessary_literal_bound,
    clippy::unnecessary_map_or,
    clippy::unused_self,
    clippy::cast_precision_loss,
    clippy::unnecessary_wraps,
    dead_code
)]

use clap::Subcommand;
use serde::{Deserialize, Serialize};

pub mod agent;
pub mod approval;
pub mod auth;
pub mod channels;
pub mod config;
pub mod cost;
pub mod cron;
pub mod daemon;
pub mod doctor;
pub mod gateway;
pub mod hardware;
pub mod health;
pub mod heartbeat;
pub mod identity;
pub mod integrations;
pub mod memory;
pub mod migration;
pub mod multimodal;
pub mod observability;
pub mod onboard;
pub mod peripherals;
pub mod providers;
pub mod rag;
pub mod runtime;
pub mod security;
pub mod service;
pub mod skills;
pub mod tools;
pub mod tunnel;
pub mod util;

pub use config::Config;

/// Service management subcommands
#[derive(Subcommand, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceCommands {
    /// Install daemon service unit for auto-start and restart
    Install,
    /// Start daemon service
    Start,
    /// Stop daemon service
    Stop,
    /// Restart daemon service to apply latest config
    Restart,
    /// Check daemon service status
    Status,
    /// Uninstall daemon service unit
    Uninstall,
}

/// Channel management subcommands
#[derive(Subcommand, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChannelCommands {
    /// List all configured channels
    List,
    /// Start all configured channels (handled in main.rs for async)
    Start,
    /// Run health checks for configured channels (handled in main.rs for async)
    Doctor,
    /// Add a new channel configuration
    #[command(long_about = "\
Add a new channel configuration.

Provide the channel type and a JSON object with the required \
configuration keys for that channel type.

Supported types: telegram, discord, slack, whatsapp, matrix, imessage, email.

Examples:
  zeroclaw channel add telegram '{\"bot_token\":\"...\",\"name\":\"my-bot\"}'
  zeroclaw channel add discord '{\"token\":\"...\",\"name\":\"my-discord\"}'
  zeroclaw channel add slack '{\"bot_token\":\"...\",\"app_token\":\"...\",\"name\":\"my-slack\"}'
  zeroclaw channel add whatsapp '{\"phone_number_id\":\"...\",\"access_token\":\"...\",\"name\":\"my-wa\"}'
  zeroclaw channel add matrix '{\"homeserver_url\":\"...\",\"username\":\"...\",\"password\":\"...\",\"name\":\"my-matrix\"}'
  zeroclaw channel add imessage '{\"name\":\"my-imessage\"}'
  zeroclaw channel add email '{\"name\":\"my-email\",\"imap\":\"...\",\"smtp\":\"...\",\"username\":\"...\",\"password\":\"...\"}'
")]
    Add {
        /// Channel type (telegram, discord, slack, whatsapp, matrix, imessage, email)
        channel_type: String,
        /// JSON string with channel configuration
        config: String,
    },
    /// Remove a channel configuration
    Remove {
        /// Channel name to remove
        name: String,
    },
}

/// Integration management subcommands
#[derive(Subcommand, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IntegrationCommands {
    /// List all available integrations
    List,
    /// Show available actions for a specific integration
    Actions {
        /// Integration name
        integration: String,
    },
    /// Configure or update an integration
    Configure {
        /// Integration name
        integration: String,
        /// JSON string with integration configuration
        config: String,
    },
    /// Remove an integration configuration
    Remove {
        /// Integration name
        integration: String,
    },
    /// Test an integration configuration
    Test {
        /// Integration name
        integration: String,
    },
}

/// Agent management subcommands
#[derive(Subcommand, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentCommands {
    /// Create a new custom agent
    Create {
        /// Agent name
        name: String,
        /// Agent description
        description: String,
        /// JSON string with agent configuration
        config: String,
    },
    /// List all custom agents
    List,
    /// Show details of a specific agent
    Show {
        /// Agent name
        name: String,
    },
    /// Update an existing agent
    Update {
        /// Agent name
        name: String,
        /// JSON string with agent configuration updates
        config: String,
    },
    /// Remove a custom agent
    Remove {
        /// Agent name
        name: String,
    },
}

/// Skill management subcommands
#[derive(Subcommand, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkillCommands {
    /// List all available skills
    List,
    /// Show details of a specific skill
    Show {
        /// Skill name
        name: String,
    },
    /// Enable a skill
    Enable {
        /// Skill name
        name: String,
    },
    /// Disable a skill
    Disable {
        /// Skill name
        name: String,
    },
}

/// Memory management subcommands
#[derive(Subcommand, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MemoryCommands {
    /// Search memories
    Search {
        /// Search query
        query: String,
    },
    /// Show recent memories
    Recent {
        /// Number of memories to show
        #[arg(default_value = "10")]
        limit: usize,
    },
    /// Clear all memories
    Clear,
}

/// RAG (Retrieval Augmented Generation) management subcommands
#[derive(Subcommand, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RagCommands {
    /// Ingest documents into RAG system
    Ingest {
        /// Path to document or directory
        path: String,
    },
    /// Search RAG system
    Search {
        /// Search query
        query: String,
        /// Number of results to return
        #[arg(default_value = "5")]
        limit: usize,
    },
    /// List all indexed documents
    List,
    /// Remove documents from RAG system
    Remove {
        /// Document ID or path
        id: String,
    },
}

/// Tool management subcommands
#[derive(Subcommand, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ToolCommands {
    /// List all available tools
    List,
    /// Show details of a specific tool
    Show {
        /// Tool name
        name: String,
    },
    /// Install a new tool
    Install {
        /// Tool name or URL
        source: String,
    },
    /// Uninstall a tool
    Uninstall {
        /// Tool name
        name: String,
    },
}

/// Provider management subcommands
#[derive(Subcommand, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderCommands {
    /// List all available providers
    List,
    /// Configure a provider
    Configure {
        /// Provider name
        provider: String,
        /// JSON string with provider configuration
        config: String,
    },
    /// Remove a provider configuration
    Remove {
        /// Provider name
        provider: String,
    },
    /// Test a provider configuration
    Test {
        /// Provider name
        provider: String,
    },
}
