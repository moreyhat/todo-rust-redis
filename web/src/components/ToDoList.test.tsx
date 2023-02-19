import React from 'react'
import { render, screen } from '@testing-library/react'
import ToDoList from './ToDoList'
import userEvent from '@testing-library/user-event'

describe('ToDoList', () => {
  test('Render ToDo items', () => {
    const items = ['The first item', 'The second item', 'The third item', 'The fourth item']

    const toDoAllItems = items.map<ToDo>((item, i) => {
      return {
        id: `render-specific-number-of-todo-items-test-${i}`,
        description: item,
      }
    })

    render(<ToDoList items={toDoAllItems} />)
    items.forEach((item) => {
      expect(screen.getByText(item)).toBeInTheDocument()
    })
  })

  test('Delete specific ToDo item', async () => {
    const items = [
      'The keep item #1',
      'The keep item #2',
      'The deletion item #3',
      'The keep item #4',
    ]
    const handleDelete = jest.fn()

    const toDoAllItems = items.map<ToDo>((item, i) => {
      return {
        id: `render-specific-number-of-todo-items-test-${i}`,
        description: item,
      }
    })

    render(<ToDoList items={toDoAllItems} handleDelete={handleDelete} />)
    expect(screen.getAllByRole('button').length).toBe(items.length)

    await userEvent.click(screen.getAllByRole('button')[2])
    expect(handleDelete).toHaveBeenNthCalledWith(1, 'render-specific-number-of-todo-items-test-2')
  })
})
