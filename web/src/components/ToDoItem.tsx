import { Typography, IconButton, Grid } from '@mui/material'
import ClearIcon from '@mui/icons-material/Clear'

type ToDoItemProps = {
  item: ToDo
  onClickDelete?: React.MouseEventHandler<HTMLButtonElement>
}

const ToDoItem = (props: ToDoItemProps) => {
  return (
    <Grid container>
      <Grid item xs={10}>
        <Typography>{props.item.description}</Typography>
      </Grid>
      <Grid item xs={2}>
        <IconButton onClick={props.onClickDelete} id={`delete-button-for-${props.item.id}`}>
          <ClearIcon />
        </IconButton>
      </Grid>
    </Grid>
  )
}

export default ToDoItem
