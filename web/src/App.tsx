import React, { useEffect, useState } from 'react'
import { Box, Typography } from '@mui/material'
import ToDoList from './components/ToDoList'
const TO_DO_API_ENDPOINT = 'http://localhost:8080'

const App = () => {
  const [items, setItems] = useState<ToDo[]>([])

  const getItems = () => {
    fetch(TO_DO_API_ENDPOINT).then((res) => {
      res.json().then((res) => {
        setItems(res as ToDo[])
      })
    })
  }

  const handleDelete = (id: string) => {
    const request = {
      method: 'DELETE',
    }
    fetch(`${TO_DO_API_ENDPOINT}/${id}`, request).then(() => {
      getItems()
    })
  }

  const handleCreate = (description: string) => {
    const todo = {
      description: description,
    }
    const request = {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json;charset=UTF-8',
      },
      body: JSON.stringify(todo),
    }
    fetch(TO_DO_API_ENDPOINT, request).then(() => {
      getItems()
    })
  }

  useEffect(() => {
    getItems()
  }, [])

  return (
    <Box width={'50%'}>
      <Typography variant='h2'>ToDo List</Typography>
      <ToDoList items={items} handleCreate={handleCreate} handleDelete={handleDelete} />
    </Box>
  )
}

export default App
