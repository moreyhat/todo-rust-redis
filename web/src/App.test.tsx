import React from 'react'
import { render, screen, waitFor, act } from '@testing-library/react'
import App from './App'
import userEvent from '@testing-library/user-event'

const TO_DO_API_ENDPOINT = process.env.TO_DO_API_ENDPOINT || 'http://localhost:8080'

describe('AppTest', () => {
  test('ToDo title is rendered', () => {
    render(<App />)
    const linkElement = screen.getByText(/ToDo List/i)
    expect(linkElement).toBeInTheDocument()
  })

  test('Display ToDo items', async () => {
    const items = [
      'The first item (Display ToDo Items)',
      'The second item (Display ToDo Items)',
      'The third item (Display ToDo Items)',
      'The fourth item (Display ToDo Items)',
    ]
    const itemIds: string[] = []

    for (const i in items) {
      const todo = {
        description: items[i],
      }
      const request = {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json;charset=UTF-8',
        },
        body: JSON.stringify(todo),
      }
      const res = await (await fetch(TO_DO_API_ENDPOINT, request)).json()
      itemIds.push(res.id)
    }

    render(<App />)
    await waitFor(() => {
      expect(screen.getByText(items[0])).toBeInTheDocument()
    })

    for (const i in items) {
      expect(screen.getByText(items[i])).toBeInTheDocument()
    }

    await act(async () => {
      for (const i in itemIds) {
        const request = {
          method: 'DELETE',
        }
        await fetch(`${TO_DO_API_ENDPOINT}/${itemIds[i]}`, request)
      }
    })
  })

  test('Delete ToDo item', async () => {
    const item = 'The delete test item on App'
    const todo = {
      description: item,
    }
    const request = {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json;charset=UTF-8',
      },
      body: JSON.stringify(todo),
    }
    const res = await (await fetch(TO_DO_API_ENDPOINT, request)).json()
    const id = res.id

    render(<App />)
    await waitFor(() => {
      expect(screen.getByText(item)).toBeInTheDocument()
    })

    const buttons = screen.getAllByRole('button')
    for (const i in buttons) {
      if (buttons[i].id === `delete-button-for-${id}`) {
        act(() => {
          userEvent.click(buttons[i])
        })
        break
      }
    }

    await waitFor(() => {
      expect(screen.queryAllByText(item).length).toBe(0)
    })
  })
})
