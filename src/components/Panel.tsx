import { useStore } from '../stores';

export function Panel() {
  const { skills, mcpServers } = useStore();

  return (
    <div className="w-64 bg-secondary/50 border-l border-border p-4">
      <div className="space-y-6">
        <div>
          <h3 className="text-sm font-semibold mb-2 text-muted-foreground">已启用技能</h3>
          <div className="space-y-1">
            {skills.filter(s => s.enabled).map((skill) => (
              <div key={skill.path} className="text-sm py-1.5 px-2 rounded bg-secondary">{skill.name}</div>
            ))}
            {skills.filter(s => s.enabled).length === 0 && (
              <p className="text-xs text-muted-foreground">暂无启用的技能</p>
            )}
          </div>
        </div>
        <div>
          <h3 className="text-sm font-semibold mb-2 text-muted-foreground">MCP 服务</h3>
          <div className="space-y-1">
            {mcpServers.map((server) => (
              <div key={server.name} className="text-sm py-1.5 px-2 rounded bg-secondary">{server.name}</div>
            ))}
            {mcpServers.length === 0 && <p className="text-xs text-muted-foreground">暂无连接的服务</p>}
          </div>
        </div>
      </div>
    </div>
  );
}
