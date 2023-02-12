import React from 'react'
import { render, screen } from '@testing-library/react'
import ToDo from './ToDo'

test('ToDo is passed as a parameter', () => {
  const description = 'Description for test'
  render(<ToDo description={description} />)
  const todoDescription = screen.getByText(description)
  expect(todoDescription).toBeInTheDocument()
})
