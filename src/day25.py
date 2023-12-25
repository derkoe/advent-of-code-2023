import networkx as nx
import matplotlib.pyplot as plt
import sys

def bridge_dfs(G,u,v,cnt,low,pre,bridges):
    cnt    += 1
    pre[v]  = cnt
    low[v]  = pre[v]

    for w in nx.neighbors(G,v):
        if (pre[w] == -1):
            bridge_dfs(G,v,w,cnt,low,pre,bridges)

            low[v] = min(low[v], low[w])
            if (low[w] == pre[w]):
                bridges.append((v,w))

        elif (w != u):
            low[v] = min(low[v], pre[w])

def get_bridges(G):
    bridges = []
    cnt     = 0
    low     = {n:-1 for n in G.nodes()}
    pre     = low.copy()

    for n in G.nodes():
         bridge_dfs(G, n, n, cnt, low, pre, bridges)

    return bridges # <- List of (node-node) tuples for all bridges in G

G = nx.Graph()

input = [line.replace('\n','').split(": ") for line in sys.stdin.readlines()]
for line in input:
    G.add_node(line[0])
    for to in line[1].split(" "):
        G.add_edge(line[0],to)

# those nodes were found using graphviz neato
G.remove_edge("ldk", "bkm")
G.remove_edge("rsm", "bvc")
G.remove_edge("zmq", "pgh")

groups = list(nx.biconnected_components(G))
print("Part1 =", len(groups[0])*len(groups[1]))

# Output in graphviz format
# for line in input:
#     for to in line[1].split(" "):
#         print(f"{line[0]} -- {to}")
