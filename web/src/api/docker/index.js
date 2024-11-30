import request from '../request'

export const getVersion = () => request.get('/api/docker/version')
