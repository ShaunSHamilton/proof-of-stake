export const tutorialState = {
  sock: () => {},
  name: "Camper",
  chain: [
    {
      data: [
        {
          name: "Camper",
          racks: 8,
          reputation: 8,
          staked: 90,
          tokens: 100,
        },
      ],
    },
  ],
  tasks: [],
  setTutorialState: null,
  listenState: false,
  transactionPool: [],
};

export const sampleTask = {
  question: "\nWhat's the correct way to display `Hello world`?\n",
  options: [
    {
      code: '```js\nconsole.log("Hello world");\n```',
      order: 0,
    },
    {
      code: '```py\nprint("Hello world")\n```',
      order: 1,
    },
    {
      code: '```c\nprintf("Hello world");\n```',
      order: 2,
    },
    {
      code: '```java\nSystem.out.println("Hello world");\n```',
      order: 3,
    },
    {
      code: '```ruby\nputs "Hello world"\n```',
      order: 4,
    },
    {
      code: "```php\n<?php echo 'Hello World'; ?>\n```",
      order: 5,
    },
  ],
};
