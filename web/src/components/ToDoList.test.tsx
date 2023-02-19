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

  test('Delete specific ToDo item', () => {
    const items = [
      'The keep item #1',
      'The keep item #2',
      'The deletion item #3',
      'The keep item #4',
    ]
    const handleDelete = jest.fn()

    const toDoItems = items.map<ToDo>((item, i) => {
      return {
        id: `render-specific-number-of-todo-items-test-${i}`,
        description: item,
      }
    })

    render(<ToDoList items={toDoItems} handleDelete={handleDelete} />)
    expect(screen.getAllByRole('button').length).toBe(items.length)

    userEvent.click(screen.getAllByRole('button')[2])
    expect(handleDelete).toHaveBeenNthCalledWith(1, 'render-specific-number-of-todo-items-test-2')
  })

  test('Create ToDo item', () => {
    const items = ['The first item', 'The second item', 'The third item', 'The fourth item']
    const handleCreate = jest.fn()

    const toDoItems = items.map<ToDo>((item, i) => {
      return {
        id: `click-create-todo-test-${i}`,
        description: item,
      }
    })

    render(<ToDoList items={toDoItems} handleCreate={handleCreate} />)
    expect(screen.getByRole('button', { name: 'Add' })).toBeInTheDocument()
    userEvent.click(screen.getByRole('button', { name: 'Add' }))
    expect(handleCreate).toHaveBeenCalledTimes(0)
    userEvent.type(screen.getByPlaceholderText('Enter todo'), 'This is new todo')
    userEvent.click(screen.getByRole('button', { name: 'Add' }))
    expect(handleCreate).toHaveBeenNthCalledWith(1, 'This is new todo')
  })
})
