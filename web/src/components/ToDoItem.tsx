import { Typography, IconButton, Grid } from '@mui/material'
import ClearIcon from '@mui/icons-material/Clear'

type ToDoProps = {
  item: ToDo
  onClick?: React.MouseEventHandler<HTMLButtonElement>
}

const ToDo = (props: ToDoProps) => {
  return (
    <Grid container>
      <Grid item xs={10}>
        <Typography>{props.item.description}</Typography>
      </Grid>
      <Grid item xs={2}>
        <IconButton onClick={props.onClick}>
          <ClearIcon />
        </IconButton>
      </Grid>
    </Grid>
  )
}

export default ToDo
