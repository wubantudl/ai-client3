use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use directories::ProjectDirs;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDefinition {
    pub name: String,
    pub description: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInfo {
    pub path: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsConfig {
    pub skills: Vec<SkillSetting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSetting {
    pub path: String,
    pub enabled: bool,
}

impl Default for SkillsConfig {
    fn default() -> Self {
        Self { skills: vec![] }
    }
}

pub fn get_skills_base_path() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "ai-client", "AI Client") {
        let skills_dir = proj_dirs.data_dir().join("skills");
        fs::create_dir_all(&skills_dir)?;
        Ok(skills_dir)
    } else {
        Err(anyhow::anyhow!("Could not determine skills directory"))
    }
}

pub fn get_skills_config_path() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "ai-client", "AI Client") {
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir)?;
        Ok(config_dir.join("skills_config.json"))
    } else {
        Err(anyhow::anyhow!("Could not determine config directory"))
    }
}

pub fn load_skills_config() -> Result<SkillsConfig> {
    let config_path = get_skills_config_path()?;
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        let config: SkillsConfig = serde_json::from_str(&content)?;
        Ok(config)
    } else {
        let config = SkillsConfig::default();
        save_skills_config(&config)?;
        Ok(config)
    }
}

pub fn save_skills_config(config: &SkillsConfig) -> Result<()> {
    let config_path = get_skills_config_path()?;
    let content = serde_json::to_string_pretty(config)?;
    fs::write(&config_path, content)?;
    Ok(())
}

pub fn list_skills() -> Result<Vec<SkillInfo>> {
    let skills_dir = get_skills_base_path()?;
    let mut skills = vec![];
    
    // 加载启用状态配置
    let config = load_skills_config().unwrap_or_default();
    
    // 扫描 skills 目录下的所有子文件夹
    if skills_dir.exists() {
        for entry in fs::read_dir(&skills_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let skill_md = path.join("skill.md");
                if skill_md.exists() {
                    // 解析 skill.md
                    if let Ok(content) = fs::read_to_string(&skill_md) {
                        let (name, description) = parse_skill_md(&content, &path);
                        let path_str = path.to_string_lossy().to_string();
                        
                        // 查找配置中的启用状态
                        let enabled = config.skills.iter()
                            .find(|s| s.path == path_str)
                            .map(|s| s.enabled)
                            .unwrap_or(true);
                        
                        skills.push(SkillInfo {
                            path: path_str,
                            name,
                            description,
                            enabled,
                        });
                    }
                }
            }
        }
    }
    
    info!("Found {} skills", skills.len());
    Ok(skills)
}

/// 解析 skill.md 文件，提取 name 和 description
fn parse_skill_md(content: &str, path: &PathBuf) -> (String, String) {
    let mut name = path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let mut description = String::new();
    
    // 尝试解析 frontmatter (YAML 格式)
    if content.starts_with("---") {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() >= 3 {
            let frontmatter = parts[1];
            for line in frontmatter.lines() {
                let line = line.trim();
                if line.starts_with("name:") {
                    name = line.strip_prefix("name:")
                        .map(|s| s.trim().to_string())
                        .unwrap_or(name);
                } else if line.starts_with("description:") {
                    description = line.strip_prefix("description:")
                        .map(|s| s.trim().to_string())
                        .unwrap_or_default();
                }
            }
        }
    }
    
    // 如果没有 description，尝试从第一行非空内容获取
    if description.is_empty() {
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with("---") && !line.starts_with("#") {
                // 去掉 markdown 标题符号
                let clean_line = line.trim_start_matches('#').trim();
                if !clean_line.is_empty() {
                    description = clean_line.to_string();
                    break;
                }
            }
        }
    }
    
    (name, description)
}

pub fn import_local_skill(source_path: &str) -> Result<SkillInfo> {
    let source = PathBuf::from(source_path);
    if !source.exists() {
        return Err(anyhow::anyhow!("Source path does not exist"));
    }

    let skills_base = get_skills_base_path()?;
    let skill_name = source.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .ok_or_else(|| anyhow::anyhow!("Invalid source path"))?;
    
    let target = skills_base.join(&skill_name);
    
    if target.exists() {
        fs::remove_dir_all(&target)?;
    }
    
    copy_dir_all(&source, &target)?;

    let mut config = load_skills_config()?;
    config.skills.push(SkillSetting {
        path: target.to_string_lossy().to_string(),
        enabled: true,
    });
    save_skills_config(&config)?;

    info!("Imported skill: {}", skill_name);

    Ok(SkillInfo {
        path: target.to_string_lossy().to_string(),
        name: skill_name,
        description: String::new(),
        enabled: true,
    })
}

pub fn import_from_github_repo(repo_url: &str) -> Result<SkillInfo> {
    info!("Importing skill from GitHub: {}", repo_url);
    
    let skills_base = get_skills_base_path()?;
    let repo_name = repo_url.split('/')
        .last()
        .ok_or_else(|| anyhow::anyhow!("Invalid GitHub URL"))?
        .trim_end_matches(".git");
    
    let target = skills_base.join(repo_name);

    let mut config = load_skills_config()?;
    config.skills.push(SkillSetting {
        path: target.to_string_lossy().to_string(),
        enabled: true,
    });
    save_skills_config(&config)?;

    Ok(SkillInfo {
        path: target.to_string_lossy().to_string(),
        name: repo_name.to_string(),
        description: format!("Skill from {}", repo_url),
        enabled: true,
    })
}

pub fn delete_skill(path: &str) -> Result<()> {
    let mut config = load_skills_config()?;
    config.skills.retain(|s| s.path != path);
    save_skills_config(&config)?;
    
    let path_buf = PathBuf::from(path);
    if path_buf.exists() {
        fs::remove_dir_all(&path_buf)?;
    }
    
    info!("Deleted skill at: {}", path);
    Ok(())
}

pub fn toggle_skill(path: &str, enabled: bool) -> Result<()> {
    let mut config = load_skills_config()?;
    
    if let Some(skill) = config.skills.iter_mut().find(|s| s.path == path) {
        skill.enabled = enabled;
        save_skills_config(&config)?;
        info!("Toggled skill {}: {}", path, enabled);
    }
    
    Ok(())
}

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}
