import { useState } from 'react'
import { Box, Button, TextField } from '@mui/material'
import ToDoItem from './ToDoItem'

type ToDoListProps = {
  items: ToDo[]
  handleCreate?: (description: string) => void
  handleDelete?: (id: string) => void
}

const ToDoList = (props: ToDoListProps) => {
  const [newItem, setNewItem] = useState('')

  const handleCreate = () => {
    if (props.handleCreate && newItem) {
      props.handleCreate(newItem)
    }
  }

  return (
    <Box>
      {props.items.map((item) => (
        <ToDoItem
          item={item}
          key={item.id}
          onClickDelete={() => {
            if (props.handleDelete) {
              props.handleDelete(item.id)
            }
          }}
        ></ToDoItem>
      ))}
      {props.handleCreate && (
        <div>
          <TextField
            id='create-item'
            value={newItem}
            placeholder='Enter todo'
            onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
              setNewItem(event.target.value)
            }}
          />
          <Button variant='text' onClick={handleCreate}>
            Add
          </Button>
        </div>
      )}
    </Box>
  )
}

export default ToDoList
