import { useState, useEffect } from 'react';
import { X, Plus, Trash2, ToggleLeft, ToggleRight, Terminal } from 'lucide-react';
import { useStore } from '../stores';
import { saveModels, deleteSkill, toggleSkill, connectMCPServer, disconnectMCPServer } from '../services/api';
import type { MCPServerConfig } from '../types';

interface SettingsProps {
  isOpen: boolean;
  onClose: () => void;
  initialTab?: Tab;
}

type Tab = 'models' | 'skills' | 'mcp';

export function Settings({ isOpen, onClose, initialTab = 'models' }: SettingsProps) {
  const { models, setModels, skills, setSkills, mcpServers, setMCPServers } = useStore();
  const [activeTab, setActiveTab] = useState<Tab>(initialTab);
  
  // 当 initialTab 变化时更新 activeTab
  useEffect(() => {
    if (isOpen) {
      setActiveTab(initialTab);
    }
  }, [isOpen, initialTab]);
  
  // MCP 表单状态
  const [mcpName, setMcpName] = useState('');
  const [mcpCommand, setMcpCommand] = useState('');
  const [mcpArgs, setMcpArgs] = useState('');

  if (!isOpen) return null;

  const handleSaveModel = async (index: number, field: string, value: string) => {
    const newModels = [...models];
    newModels[index] = { ...newModels[index], [field]: value };
    setModels(newModels);
    await saveModels(newModels);
  };

  const handleDeleteSkill = async (path: string) => {
    try {
      await deleteSkill(path);
      setSkills(skills.filter(s => s.path !== path));
    } catch (error) {
      alert('删除失败: ' + error);
    }
  };

  const handleToggleSkill = async (path: string, enabled: boolean) => {
    try {
      await toggleSkill(path, !enabled);
      setSkills(skills.map(s => s.path === path ? { ...s, enabled: !enabled } : s));
    } catch (error) {
      alert('切换失败: ' + error);
    }
  };

  const handleAddMCPServer = async () => {
    if (!mcpName.trim() || !mcpCommand.trim()) return;
    try {
      const config: MCPServerConfig = {
        name: mcpName.trim(),
        transport: 'stdio',
        command: mcpCommand.trim(),
        args: mcpArgs.trim() ? mcpArgs.trim().split(' ') : undefined,
      };
      await connectMCPServer(config);
      setMCPServers([...mcpServers, config]);
      setMcpName('');
      setMcpCommand('');
      setMcpArgs('');
    } catch (error) {
      alert('添加失败: ' + error);
    }
  };

  const handleRemoveMCPServer = async (name: string) => {
    try {
      await disconnectMCPServer(name);
      setMCPServers(mcpServers.filter(s => s.name !== name));
    } catch (error) {
      alert('移除失败: ' + error);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-card border border-border rounded-lg w-[700px] max-h-[80vh] overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-border">
          <h2 className="text-lg font-semibold">设置</h2>
          <button onClick={onClose} className="hover:bg-secondary p-1 rounded">
            <X size={20} />
          </button>
        </div>
        
        {/* Tabs */}
        <div className="flex border-b border-border">
          <button
            onClick={() => setActiveTab('models')}
            className={`px-4 py-2 text-sm font-medium transition-colors ${
              activeTab === 'models' 
                ? 'border-b-2 border-primary text-primary' 
                : 'text-muted-foreground hover:text-foreground'
            }`}
          >
            模型配置
          </button>
          <button
            onClick={() => setActiveTab('skills')}
            className={`px-4 py-2 text-sm font-medium transition-colors ${
              activeTab === 'skills' 
                ? 'border-b-2 border-primary text-primary' 
                : 'text-muted-foreground hover:text-foreground'
            }`}
          >
            技能管理
          </button>
          <button
            onClick={() => setActiveTab('mcp')}
            className={`px-4 py-2 text-sm font-medium transition-colors ${
              activeTab === 'mcp' 
                ? 'border-b-2 border-primary text-primary' 
                : 'text-muted-foreground hover:text-foreground'
            }`}
          >
            MCP服务
          </button>
        </div>
        
        {/* Content */}
        <div className="p-4 overflow-y-auto max-h-[calc(80vh-120px)]">
          {/* 模型配置 */}
          {activeTab === 'models' && (
            <div className="space-y-3">
              {models.map((model, index) => (
                <div key={model.id} className="p-3 bg-secondary rounded-lg space-y-2">
                  <div className="flex items-center justify-between">
                    <span className="font-medium">{model.name}</span>
                    <span className="text-xs text-muted-foreground">{model.type}</span>
                  </div>
                  <div className="grid grid-cols-2 gap-2">
                    <input
                      type="text"
                      placeholder="Base URL"
                      value={model.base_url}
                      onChange={(e) => handleSaveModel(index, 'base_url', e.target.value)}
                      className="bg-background border border-border rounded px-2 py-1 text-sm"
                    />
                    <input
                      type="password"
                      placeholder="API Key"
                      value={model.api_key}
                      onChange={(e) => handleSaveModel(index, 'api_key', e.target.value)}
                      className="bg-background border border-border rounded px-2 py-1 text-sm"
                    />
                  </div>
                </div>
              ))}
            </div>
          )}
          
          {/* 技能管理 */}
          {activeTab === 'skills' && (
            <div className="space-y-4">
              {/* 提示信息 */}
              <div className="p-3 bg-secondary rounded-lg">
                <h4 className="text-sm font-medium mb-2">技能目录</h4>
                <p className="text-xs text-muted-foreground">
                  将技能文件夹放到 skills 目录下，每个技能需包含 skill.md 文件
                </p>
                <p className="text-xs text-muted-foreground mt-1">
                  skill.md 格式：在文件开头使用 --- 包裹的 frontmatter 定义 name 和 description
                </p>
              </div>
              
              {/* 技能列表 */}
              <div className="space-y-2">
                {skills.length === 0 ? (
                  <div className="text-center text-muted-foreground py-4">
                    暂无技能，请将技能文件夹放入 skills 目录
                  </div>
                ) : (
                  skills.map((skill) => (
                    <div key={skill.path} className="flex items-center justify-between p-3 bg-secondary rounded-lg">
                      <div className="flex-1">
                        <div className="font-medium">{skill.name}</div>
                        <div className="text-xs text-muted-foreground">{skill.description || '无描述'}</div>
                      </div>
                      <div className="flex items-center gap-2">
                        <button
                          onClick={() => handleToggleSkill(skill.path, skill.enabled)}
                          className="p-1 hover:bg-secondary-foreground/10 rounded"
                          title={skill.enabled ? '禁用' : '启用'}
                        >
                          {skill.enabled ? (
                            <ToggleRight size={20} className="text-green-500" />
                          ) : (
                            <ToggleLeft size={20} className="text-muted-foreground" />
                          )}
                        </button>
                        <button
                          onClick={() => handleDeleteSkill(skill.path)}
                          className="p-1 hover:bg-destructive/20 rounded text-destructive"
                          title="删除"
                        >
                          <Trash2 size={18} />
                        </button>
                      </div>
                    </div>
                  ))
                )}
              </div>
            </div>
          )}
          
          {/* MCP服务 */}
          {activeTab === 'mcp' && (
            <div className="space-y-4">
              {/* 添加表单 */}
              <div className="p-3 bg-secondary rounded-lg space-y-3">
                <h4 className="text-sm font-medium">添加 MCP Server</h4>
                <div className="grid grid-cols-2 gap-2">
                  <input
                    type="text"
                    placeholder="名称"
                    value={mcpName}
                    onChange={(e) => setMcpName(e.target.value)}
                    className="bg-background border border-border rounded px-2 py-1 text-sm"
                  />
                  <input
                    type="text"
                    placeholder="命令 (如: npx)"
                    value={mcpCommand}
                    onChange={(e) => setMcpCommand(e.target.value)}
                    className="bg-background border border-border rounded px-2 py-1 text-sm"
                  />
                </div>
                <div className="flex gap-2">
                  <input
                    type="text"
                    placeholder="参数 (空格分隔)"
                    value={mcpArgs}
                    onChange={(e) => setMcpArgs(e.target.value)}
                    className="flex-1 bg-background border border-border rounded px-2 py-1 text-sm"
                  />
                  <button
                    onClick={handleAddMCPServer}
                    className="flex items-center gap-1 px-3 py-1 bg-primary text-primary-foreground rounded text-sm hover:bg-primary/90"
                  >
                    <Plus size={14} />
                    添加
                  </button>
                </div>
              </div>
              
              {/* 服务列表 */}
              <div className="space-y-2">
                {mcpServers.length === 0 ? (
                  <div className="text-center text-muted-foreground py-4">
                    暂无 MCP 服务，请添加
                  </div>
                ) : (
                  mcpServers.map((server) => (
                    <div key={server.name} className="flex items-center justify-between p-3 bg-secondary rounded-lg">
                      <div className="flex items-center gap-2">
                        <Terminal size={18} className="text-muted-foreground" />
                        <div>
                          <div className="font-medium">{server.name}</div>
                          <div className="text-xs text-muted-foreground">
                            {server.command} {server.args?.join(' ')}
                          </div>
                        </div>
                      </div>
                      <button
                        onClick={() => handleRemoveMCPServer(server.name)}
                        className="p-1 hover:bg-destructive/20 rounded text-destructive"
                        title="移除"
                      >
                        <Trash2 size={18} />
                      </button>
                    </div>
                  ))
                )}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
