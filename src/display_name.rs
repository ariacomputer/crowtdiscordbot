use serenity::model::channel::Message;
use serenity::prelude::*;
use regex::Regex;

// Helper function to get the best display name for a user
pub async fn get_best_display_name(ctx: &Context, msg: &Message) -> String {
    // Prioritize server nickname over global name over username
    if let Some(guild_id) = msg.guild_id {
        // Get member data which includes the nickname
        if let Ok(member) = guild_id.member(&ctx.http, msg.author.id).await {
            // Use nickname if available, otherwise fall back to global name or username
            if let Some(nick) = member.nick {
                return nick;
            }
        }
    }
    
    // Fall back to global name or username if no nickname or couldn't get member data
    msg.author.global_name.clone().unwrap_or_else(|| msg.author.name.clone())
}

// Clean a display name by removing IRC formatting, brackets, and pronouns
pub fn clean_display_name(name: &str) -> String {
    // First remove IRC formatting
    let mut clean_name = name.to_string();
    clean_name = clean_name.replace("[irc]", "").trim().to_string();
    
    // Check for pronouns in parentheses (they/them)
    if let Some(idx) = clean_name.find('(') {
        let after_paren = &clean_name[idx..];
        if after_paren.contains("/") || 
           after_paren.contains("he") || 
           after_paren.contains("she") || 
           after_paren.contains("they") || 
           after_paren.contains("xe") || 
           after_paren.contains("ze") ||
           after_paren.contains("it") || 
           after_paren.contains("fae") {
            clean_name = clean_name[0..idx].trim().to_string();
        }
    }
    
    // Check for pronouns in brackets [she/her]
    if let Some(idx) = clean_name.find('[') {
        let after_bracket = &clean_name[idx..];
        if after_bracket.contains("/") || 
           after_bracket.contains("he") || 
           after_bracket.contains("she") || 
           after_bracket.contains("they") || 
           after_bracket.contains("xe") || 
           after_bracket.contains("ze") ||
           after_bracket.contains("it") || 
           after_bracket.contains("fae") {
            clean_name = clean_name[0..idx].trim().to_string();
        }
    }
    
    // Check for pronouns in angle brackets <any/all>
    if let Some(idx) = clean_name.find('<') {
        let after_angle = &clean_name[idx..];
        if after_angle.contains("/") || 
           after_angle.contains("he") || 
           after_angle.contains("she") || 
           after_angle.contains("they") || 
           after_angle.contains("xe") || 
           after_angle.contains("ze") ||
           after_angle.contains("it") || 
           after_angle.contains("fae") {
            clean_name = clean_name[0..idx].trim().to_string();
        }
    }
    
    clean_name.trim().to_string()
}

// Extract pronouns from a display name
pub fn extract_pronouns(name: &str) -> Option<String> {
    // Check for pronouns in parentheses (they/them)
    let parentheses_regex = Regex::new(r"\(([^)]*)\)").ok()?;
    if let Some(captures) = parentheses_regex.captures(name) {
        let content = captures.get(1)?.as_str().to_lowercase();
        if content.contains("/") || content.contains("he") || content.contains("she") || 
           content.contains("they") || content.contains("xe") || content.contains("ze") ||
           content.contains("it") || content.contains("fae") {
            return Some(content);
        }
    }
    
    // Check for pronouns in brackets [she/her]
    let brackets_regex = Regex::new(r"\[([^\]]*)\]").ok()?;
    if let Some(captures) = brackets_regex.captures(name) {
        let content = captures.get(1)?.as_str().to_lowercase();
        if content.contains("/") || content.contains("he") || content.contains("she") || 
           content.contains("they") || content.contains("xe") || content.contains("ze") ||
           content.contains("it") || content.contains("fae") {
            return Some(content);
        }
    }
    
    // Check for pronouns in angle brackets <any/all>
    let angles_regex = Regex::new(r"<([^>]*)>").ok()?;
    if let Some(captures) = angles_regex.captures(name) {
        let content = captures.get(1)?.as_str().to_lowercase();
        if content.contains("/") || content.contains("he") || content.contains("she") || 
           content.contains("they") || content.contains("xe") || content.contains("ze") ||
           content.contains("it") || content.contains("fae") {
            return Some(content);
        }
    }
    
    None
}

// Get a clean display name with all formatting and pronouns removed
pub async fn get_clean_display_name(ctx: &Context, msg: &Message) -> String {
    let display_name = get_best_display_name(ctx, msg).await;
    clean_display_name(&display_name)
}
