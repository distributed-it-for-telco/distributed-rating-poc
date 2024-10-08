# Rating Coordinator
This actor serves as a gateway to the multiple rating agents assumed to be running in the rating agent lattice. It takes
a single incoming rating request and farms it out to all applicable rating agents and returns the aggregate result.

