import request from '../request'

export const getVersion = () => request.get('/api/docker/version')
export const getPackages = () => request.get('/api/docker/packages')
export const installPackage = name => request.post('/api/docker/install/' + name)
