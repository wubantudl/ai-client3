import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { ModelConfig, ChatMessage, SkillInfo, MCPServerConfig } from '../types';

export const getModels = () => invoke<ModelConfig[]>('get_models');
export const saveModels = (models: ModelConfig[]) => invoke('save_model_config', { models });
export const chat = (modelId: string, messages: ChatMessage[]) => invoke<string>('chat', { modelId, messages });

export const chatStream = async (modelId: string, messages: ChatMessage[], onChunk: (c: string) => void, onDone: () => void) => {
  const unlisten = await listen<{ content: string }>('stream-chunk', (e) => onChunk(e.payload.content));
  const unlistenDone = await listen('stream-done', () => onDone());
  try {
    await invoke('chat_stream', { modelId, messages });
  } finally {
    unlisten();
    unlistenDone();
  }
};

export const listSkills = () => invoke<SkillInfo[]>('list_skills');
export const importSkill = (sourcePath: string) => invoke<SkillInfo>('import_skill', { sourcePath });
export const importSkillFromGitHub = (repoUrl: string) => invoke<SkillInfo>('import_from_github', { repoUrl });
export const deleteSkill = (path: string) => invoke('delete_skill', { path });
export const toggleSkill = (path: string, enabled: boolean) => invoke('toggle_skill', { path, enabled });
export const getSkillPath = () => invoke<string>('get_skill_path');

export const listMCPServers = () => invoke<MCPServerConfig[]>('list_servers');
export const connectMCPServer = (config: MCPServerConfig) => invoke('connect_server', { config });
export const disconnectMCPServer = (name: string) => invoke('disconnect_server', { name });
export const callMCPTool = (serverName: string, toolName: string, args: Record<string, unknown>) => 
  invoke('call_tool', { serverName, toolName, arguments: args });
