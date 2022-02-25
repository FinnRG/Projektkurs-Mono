import { Card } from "react-bulma-components"
import { Link } from "react-router-dom"

const TagLink = ({ id, name }) => {
    return <Card>
        <Card.Content>
            <Link to={'/tag/' + id}>
                {name}
            </Link>
        </Card.Content>
    </Card>
}

export default TagLink;