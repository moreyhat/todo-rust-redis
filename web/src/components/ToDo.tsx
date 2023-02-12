type ToDoProps = {
  description: string
}

const ToDo = (props: ToDoProps) => {
  return <div>{props.description}</div>
}

export default ToDo
