import { Box } from '@mui/material'
import ToDoItem from './ToDoItem'

type ToDoProps = {
  items: ToDo[]
}

const ToDoList = (props: ToDoProps) => {
  return (
    <Box>
      {props.items.map((item) => (
        <ToDoItem item={item} key={item.id}></ToDoItem>
      ))}
    </Box>
  )
}

export default ToDoList
