package server

type idMaker struct {
	seed int64
}

func newIdMaker() idMaker {
	return idMaker{
		seed: 1,
	}
}

func (idm *idMaker) next() int64 {
	val := idm.seed
	idm.seed += 1
	return val
}
