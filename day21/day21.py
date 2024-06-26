import numpy as np
P = complex
class Grid:
	def __init__(self, input):
		self.size = len(input.splitlines())
		self.grid = set()
		self.positions = {}
		self.frontier = set()
		self.parity = False
		for y, l in enumerate(input.splitlines()):
			for x, v in enumerate(l):
				if v == '#':
					self.grid.add(P(x,y))
				if v == 'S':
					self.positions[P(x,y)]=self.parity
					self.frontier.add(P(x,y))
	def score(self):
		return sum(v==self.parity for v in self.positions.values())
	def step(self):
		newFrontier = set()
		self.parity = not self.parity
		for p in self.frontier:
			for d in (1,-1,1j,-1j):
				if self.wrap(p+d) not in self.grid and p+d not in self.positions:
					newFrontier.add(p+d)
					self.positions[p+d] = self.parity
		self.frontier = newFrontier
	def wrap(self, p):
		return P(p.real%self.size, p.imag%self.size)
	def render(self, name):
		import matplotlib.pyplot as plt
		from matplotlib import colors
		cmap = colors.ListedColormap(['white', 'black', 'green','red'])
		grids = 5
		H = np.zeros((self.size*grids,self.size*grids))
		for p in self.grid:
			for y in range(grids):
				for x in range(grids):
					H[y*self.size + int(p.imag), x*self.size + int(p.real)] = 1
		for p,parity in self.positions.items():
			try:
				H[int(p.imag)+self.size*grids//2- self.size//2, int(p.real)+self.size*grids//2 - self.size//2] = parity*2 + 2
			except IndexError:
				...
		plt.title("Step: "+str(name)+", score: "+str(self.score()))
		plt.imshow(H, interpolation='none', cmap=cmap)
		plt.savefig(f"step{name}.png")

def main(input):
	g = Grid(input)
	# f(x) = how many squares are visited at time 65 + 131*x
	X,Y = [0,1,2], []
	target = (26501365 - 65)//131
	for s in range(65 + 131*2 + 1):
		if s%131 == 65:
			Y.append(g.score())
		# if s%131 == 65 or s%131 == 0:
		# 	g.render(s)
		if s == 64:
			p1 = g.score()
		g.step()
	poly = np.rint(np.polynomial.polynomial.polyfit(X,Y,2)).astype(int).tolist()
	return p1, sum(poly[i]*target**i for i in range(3))
