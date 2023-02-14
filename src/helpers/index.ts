export type Node = {
  id: number;
  title: string;
  description: string;
  children: Array<Node>;
};

export type Roadmap = {
  id: number;
  title: string;
  nodes: Array<Node>;
};

export function loadAllRoadmaps(): Array<Roadmap> {
  // placeholder for testing
  return [
    {
      id: 1,
      title: 'Roadmap 1',
      nodes: [
        {
          id: 3,
          title: 'Main 1',
          description: '',
          children: [
            { id: 10, title: 'Sub 1', description: 'Do this and that', children: [] },
            { id: 11, title: 'Sub 2', description: 'Simple description', children: [] },
          ],
        },
        {
          id: 4,
          title: 'Main 2',
          description: 'Very important step! Make sure to follow this one!',
          children: [],
        },
        { id: 5, title: 'Main 3', description: '', children: [] },
      ],
    },
    {
      id: 2,
      title: 'Roadmap 2',
      nodes: [
        { id: 6, title: 'Main 1', description: '', children: [] },
        {
          id: 7,
          title: 'Main 2',
          description: '',
          children: [{ id: 9, title: 'Sub 1', description: '', children: [] }],
        },
        { id: 8, title: 'Main 3', description: '', children: [] },
      ],
    },
  ];
}

export {};
