import { useEffect, useState } from "react"
import TagLink from "../../components/tag/TagLink";
import client from "../../global/client";

const TagList = () => {

    const [tags, setTags] = useState([]);

    useEffect(() => {
        client.get('/tag/get', {
            withCredentials: false,
        }).then((resp) => setTags(resp.data));
    }, []);

    return <>
        <p className='title'>Tags</p>
        {tags.map((tag, index) => (
            <TagLink key={index} id={tag.id} name={tag.name} />
        ))}
    </>
}

export default TagList;