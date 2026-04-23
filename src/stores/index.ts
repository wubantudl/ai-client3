import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { ModelConfig, Conversation, ChatMessage, SkillInfo, MCPServerConfig } from '../types';

interface AppState {
  models: ModelConfig[];
  activeModelId: string | null;
  setModels: (models: ModelConfig[]) => void;
  setActiveModel: (modelId: string) => void;
  conversations: Conversation[];
  activeConversationId: string | null;
  addConversation: () => void;
  setActiveConversation: (id: string) => void;
  addMessage: (id: string, msg: ChatMessage) => void;
  updateLastMessage: (id: string, content: string) => void;
  skills: SkillInfo[];
  setSkills: (skills: SkillInfo[]) => void;
  mcpServers: MCPServerConfig[];
  setMCPServers: (servers: MCPServerConfig[]) => void;
}

export const useStore = create<AppState>()(
  persist(
    (set, get) => ({
      models: [],
      activeModelId: null,
      setModels: (models) => set({ models }),
      setActiveModel: (modelId) => set({ activeModelId: modelId }),
      conversations: [],
      activeConversationId: null,
      addConversation: () => {
        const conv: Conversation = {
          id: crypto.randomUUID(),
          title: '新对话',
          messages: [],
          modelId: get().activeModelId || 'deepseek-chat',
        };
        set((s) => ({
          conversations: [conv, ...s.conversations],
          activeConversationId: conv.id,
        }));
      },
      setActiveConversation: (id) => set({ activeConversationId: id }),
      addMessage: (id, msg) => set((s) => ({
        conversations: s.conversations.map((c) =>
          c.id === id
            ? { ...c, messages: [...c.messages, msg], title: c.messages.length === 0 && msg.role === 'user' ? msg.content.slice(0, 30) : c.title }
            : c
        ),
      })),
      updateLastMessage: (id, content) => set((s) => ({
        conversations: s.conversations.map((c) => {
          if (c.id !== id) return c;
          const msgs = [...c.messages];
          if (msgs.length > 0) {
            msgs[msgs.length - 1] = { ...msgs[msgs.length - 1], content: msgs[msgs.length - 1].content + content };
          }
          return { ...c, messages: msgs };
        }),
      })),
      skills: [],
      setSkills: (skills) => set({ skills }),
      mcpServers: [],
      setMCPServers: (servers) => set({ mcpServers: servers }),
    }),
    { name: 'ai-client-storage' }
  )
);
