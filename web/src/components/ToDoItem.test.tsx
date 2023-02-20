import React from 'react'
import { render, screen } from '@testing-library/react'
import ToDoItem from './ToDoItem'
import userEvent from '@testing-library/user-event'

describe('ToDoItem', () => {
  test('ToDo description is passed as a prop', () => {
    const description = 'Description for test'
    const todo: ToDo = {
      id: 'ToDoItem-test-1',
      description: description,
    }
    render(<ToDoItem item={todo} onClickDelete={jest.fn()} />)
    const component = screen.getByText(description)
    expect(component).toBeInTheDocument()
  })

  test('Delete button is in the Document', () => {
    const description = 'Check delete button'
    const todo: ToDo = {
      id: 'ToDoItem-test-2',
      description: description,
    }
    render(<ToDoItem item={todo} onClickDelete={jest.fn()} />)

    const button = screen.getByRole('button')
    expect(button).toBeInTheDocument()
    expect(button).toBeEnabled()
  })

  test('Click delete triggers event', () => {
    const description = 'Delete event trigger check'
    const todo: ToDo = {
      id: 'ToDoItem-test-3',
      description: description,
    }
    const onClick = jest.fn()
    render(<ToDoItem item={todo} onClickDelete={onClick} />)

    userEvent.click(screen.getByRole('button'))
    expect(onClick).toHaveBeenCalledTimes(1)
  })
})
