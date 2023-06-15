// This file aims to serve the GraphQL /invoice route

module.exports = {
  billings: [
    {
      id: "1",
      generated_at: "2023-06-06T13:23:04+00:00",
      timestamp: "2023-06-07T13:23:04+00:00",
      items: [
        {
          id: "1",
          price: 100,
          name: "cpu",
          value: .5,
          treated: true,
          timestamp: "2023-06-06T13:23:04+00:00"
        },
        {
          id: "2",
          price: 50,
          name: "ram",
          value: .5,
          treated: true,
          timestamp: "2023-06-07T13:23:04+00:00"
        },
        {
          id: "3",
          price: 500,
          name: "storage",
          value: 20,
          treated: true,
          timestamp: "2023-06-08T13:23:04+00:00"
        },

      ],
    },
    {
      id: "2",
      generated_at: "2023-05-06T13:23:04+00:00",
      timestamp: "2023-05-07T13:23:04+00:00",
      items: [
        {
          id: "1",
          price: 800,
          name: "network",
          value: 500,
          treated: true,
          timestamp: "2023-06-06T13:23:04+00:00"
        },
        {
          id: "2",
          price: 100,
          name: "ram",
          value: 20,
          treated: true,
          timestamp: "2023-06-07T13:23:04+00:00"
        },
        {
          id: "3",
          price: 800,
          name: "storage",
          value: 20,
          treated: true,
          timestamp: "2023-06-08T13:23:04+00:00"
        },

      ],
    },
    {
      id: "3",
      generated_at: "2023-04-06T13:23:04+00:00",
      timestamp: "2023-04-07T13:23:04+00:00",
      items: [
        {
          id: "1",
          price: 400,
          name: "network",
          value: 500,
          treated: true,
          timestamp: "2023-06-06T13:23:04+00:00"
        },
        {
          id: "2",
          price: 200,
          name: "ram",
          value: 20,
          treated: true,
          timestamp: "2023-06-07T13:23:04+00:00"
        },
        {
          id: "3",
          price: 500,
          name: "storage",
          value: 20,
          treated: true,
          timestamp: "2023-06-08T13:23:04+00:00"
        },

      ],
    },
    {
      id: "4",
      generated_at: "2023-03-06T13:23:04+00:00",
      timestamp: "2023-03-07T13:23:04+00:00",
      items: [
        {
          id: "1",
          price: 150,
          name: "network",
          value: 500,
          treated: true,
          timestamp: "2023-06-06T13:23:04+00:00"
        },
        {
          id: "2",
          price: 200,
          name: "ram",
          value: 20,
          treated: true,
          timestamp: "2023-06-07T13:23:04+00:00"
        },
        {
          id: "3",
          price: 500,
          name: "storage",
          value: 20,
          treated: true,
          timestamp: "2023-06-08T13:23:04+00:00"
        },

      ],
    },
    {
      id: "5",
      generated_at: "2023-02-06T13:23:04+00:00",
      timestamp: "2023-02-07T13:23:04+00:00",
      items: [
        {
          id: "1",
          price: 450,
          name: "network",
          value: 5000,
          treated: true,
          timestamp: "2023-06-06T13:23:04+00:00"
        },


      ],
    },
    {
      id: "5",
      generated_at: "2023-01-06T13:23:04+00:00",
      timestamp: "2023-01-07T13:23:04+00:00",
      items: [
        {
          id: "1",
          price: 250,
          name: "network",
          value: 500,
          treated: true,
          timestamp: "2023-06-06T13:23:04+00:00"
        },
        {
          id: "3",
          price: 500,
          name: "storage",
          value: 20,
          treated: true,
          timestamp: "2023-06-08T13:23:04+00:00"
        },

      ],
    },
  ]
}