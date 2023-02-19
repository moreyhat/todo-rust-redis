import { Box } from '@mui/material'
import ToDoItem from './ToDoItem'

type ToDoListProps = {
  items: ToDo[]
  handleDelete?: (id: string) => void
}

const ToDoList = (props: ToDoListProps) => {
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
    </Box>
  )
}

export default ToDoList
