export const groups = [
	{
		name: 'head',
		origin: [0, 24, 0]
	},
	{
		name: 'body',
		origin: [0, 24, 0]
	},
	{
		name: 'leftArm',
		origin: [-5, 22, 0]
	},
	{
		name: 'rightArm',
		origin: [5, 22, 0]
	},
	{
		name: 'leftLeg',
		origin: [-1.95, 12, 0]
	},
	{
		name: 'rightLeg',
		origin: [1.95, 12, 0]
	}
] as GroupOptions[];

export const groupNames = groups.map((e) => e.name);

export const cubes = [
	{
		//Head
		size: [8, 8, 8],
		pos: [0, 28, 0],
		origin: [0, 24, 0],
		north: { uv: [2.032, 2.032, 3.968, 3.968] },
		east: { uv: [0.032, 2.032, 1.968, 3.968] },
		south: { uv: [6.032, 2.032, 7.968, 3.968] },
		west: { uv: [4.032, 2.032, 5.968, 3.968] },
		up: { uv: [3.968, 1.968, 2.032, 0.032] },
		down: { uv: [5.968, 0.032, 4.032, 1.968] }
	},
	{
		//Body
		size: [8, 12, 4],
		pos: [0, 18, 0],
		north: { uv: [5.032, 5.032, 6.968, 7.968] },
		east: { uv: [4.032, 5.032, 4.968, 7.968] },
		south: { uv: [8.032, 5.032, 9.968, 7.968] },
		west: { uv: [7.032, 5.032, 7.968, 7.968] },
		up: { uv: [5.032, 4.968, 6.968, 4.032] },
		down: { uv: [7.032, 4.032, 8.968, 4.968] }
	},
	{
		//L Arm
		size: [4, 12, 4],
		pos: [-6, 18, 0],
		origin: [-4, 22, 0],
		north: { uv: [9.032, 13.032, 9.968, 15.968] },
		east: { uv: [8.032, 13.032, 8.968, 15.968] },
		south: { uv: [11.032, 13.032, 11.968, 15.968] },
		west: { uv: [10.032, 13.032, 10.968, 15.968] },
		up: { uv: [9.968, 12.968, 9.032, 12.032] },
		down: { uv: [10.968, 12.032, 10.032, 12.968] }
	},
	{
		//R Arm
		size: [4, 12, 4],
		pos: [6, 18, 0],
		origin: [4, 22, 0],
		north: { uv: [11.032, 5.032, 11.968, 7.968] },
		east: { uv: [10.032, 5.032, 10.968, 7.968] },
		south: { uv: [13.032, 5.032, 13.968, 7.968] },
		west: { uv: [12.032, 5.032, 12.968, 7.968] },
		up: { uv: [11.968, 4.968, 11.032, 4.032] },
		down: { uv: [12.968, 4.032, 12.032, 4.968] }
	},
	{
		//L Leg
		size: [3.95, 12, 4],
		pos: [-1.975, 6, 0],
		origin: [0, 12, 0],
		north: { uv: [5.032, 13.032, 5.968, 15.968] },
		east: { uv: [4.032, 13.032, 4.968, 15.968] },
		south: { uv: [7.032, 13.032, 7.968, 15.968] },
		west: { uv: [6.032, 13.032, 6.968, 15.968] },
		up: { uv: [5.968, 12.968, 5.032, 12.032] },
		down: { uv: [6.968, 12.032, 6.032, 12.968] }
	},
	{
		//R Leg
		size: [3.95, 12, 4],
		pos: [1.975, 6, 0],
		origin: [0, 12, 0],
		north: { uv: [1.032, 5.032, 1.968, 7.968] },
		east: { uv: [0.032, 5.032, 0.968, 7.968] },
		south: { uv: [3.032, 5.032, 3.968, 7.968] },
		west: { uv: [2.032, 5.032, 2.968, 7.968] },
		up: { uv: [1.968, 4.968, 1.032, 4.032] },
		down: { uv: [2.968, 4.032, 2.032, 4.968] }
	}
];
