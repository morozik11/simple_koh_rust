/*type actions*/
export const ADD = 'ADD';
export const DELETE = 'DELETE';
export const GET = 'GET';

/*action generators*/
export function addElem(file){
	return { type: ADD, file}
}

export function deleteElem(name) {
	return { type: DELETE, name}
}

export function getElem(name){
    return {tep: GET, name}
}

