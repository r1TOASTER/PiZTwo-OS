/*
    File description:
    
    needed for implementing the EVTs ISRs (what happen when an exception actually happens?)
    handlers implementation over here

    TODO: Check for secure / non-secure and privileged levels and ES
    TODO: if needed, modify ELR_ELx - the preferred exception return address (in SVC - sync I think it should be +4 bytes (pointing at the SVC?))
*/