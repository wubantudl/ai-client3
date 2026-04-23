import { Plus, MessageSquare, Settings, Puzzle, Plug } from 'lucide-react';
import { useStore } from '../stores';

interface SidebarProps {
  onOpenSettings: () => void;
  onOpenSkills: () => void;
  onOpenMcp: () => void;
}

export function Sidebar({ onOpenSettings, onOpenSkills, onOpenMcp }: SidebarProps) {
  const { conversations, activeConversationId, addConversation, setActiveConversation } = useStore();

  return (
    <div className="w-64 bg-secondary/50 border-r border-border flex flex-col">
      <div className="p-3">
        <button onClick={addConversation} className="w-full flex items-center gap-2 px-3 py-2 rounded-lg bg-primary text-primary-foreground hover:bg-primary/90">
          <Plus size={18} />
          <span>新建对话</span>
        </button>
      </div>
      
      <div className="flex-1 overflow-y-auto p-2 space-y-1">
        {conversations.map((conv) => (
          <div
            key={conv.id}
            onClick={() => setActiveConversation(conv.id)}
            className={`flex items-center gap-2 px-3 py-2 rounded-lg cursor-pointer transition-colors ${
              activeConversationId === conv.id ? 'bg-primary/20 text-primary' : 'hover:bg-secondary'
            }`}
          >
            <MessageSquare size={16} />
            <span className="truncate text-sm">{conv.title}</span>
          </div>
        ))}
      </div>
      
      <div className="p-2 border-t border-border space-y-1">
        <button onClick={onOpenSkills} className="w-full flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-secondary transition-colors text-sm">
          <Puzzle size={16} />
          <span>技能管理</span>
        </button>
        <button onClick={onOpenMcp} className="w-full flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-secondary transition-colors text-sm">
          <Plug size={16} />
          <span>MCP 服务</span>
        </button>
        <button onClick={onOpenSettings} className="w-full flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-secondary transition-colors text-sm">
          <Settings size={16} />
          <span>设置</span>
        </button>
      </div>
    </div>
  );
}
