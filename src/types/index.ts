export interface ModelConfig {
  id: string;
  name: string;
  base_url: string;
  api_key: string;
  type: 'online' | 'local';
}

export interface ChatMessage {
  role: 'user' | 'assistant' | 'system';
  content: string;
}

export interface Conversation {
  id: string;
  title: string;
  messages: ChatMessage[];
  modelId: string;
}

export interface SkillInfo {
  path: string;
  name: string;
  description: string;
  enabled: boolean;
}

export interface MCPServerConfig {
  name: string;
  transport: 'stdio' | 'sse';
  command?: string;
  args?: string[];
  url?: string;
}
