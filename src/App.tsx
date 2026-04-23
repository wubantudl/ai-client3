import { useState, useEffect } from 'react';
import { Sidebar } from './components/Sidebar';
import { Chat } from './components/Chat';
import { Panel } from './components/Panel';
import { Settings } from './components/Settings';
import { useStore } from './stores';
import { getModels, listSkills, listMCPServers } from './services/api';

function App() {
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [settingsTab, setSettingsTab] = useState<'models' | 'skills' | 'mcp'>('models');
  const { models, setModels, activeModelId, setActiveModel, setSkills, setMCPServers, addConversation, conversations } = useStore();

  useEffect(() => {
    const init = async () => {
      try {
        const loadedModels = await getModels();
        setModels(loadedModels);
        
        // 设置默认模型
        if (!activeModelId && loadedModels.length > 0) {
          setActiveModel(loadedModels[0].id);
        }
        
        setSkills(await listSkills());
        setMCPServers(await listMCPServers());
        if (conversations.length === 0) addConversation();
      } catch (error) {
        console.error('Init error:', error);
      }
    };
    init();
  }, []);

  const openSettingsWithTab = (tab: 'models' | 'skills' | 'mcp') => {
    setSettingsTab(tab);
    setSettingsOpen(true);
  };

  return (
    <div className="flex h-screen bg-background text-foreground">
      <Sidebar 
        onOpenSettings={() => openSettingsWithTab('models')}
        onOpenSkills={() => openSettingsWithTab('skills')}
        onOpenMcp={() => openSettingsWithTab('mcp')}
      />
      <Chat />
      <Panel />
      <Settings 
        isOpen={settingsOpen} 
        onClose={() => setSettingsOpen(false)} 
        initialTab={settingsTab}
      />
    </div>
  );
}

export default App;
