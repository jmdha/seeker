pub mod heuristic;

mod gripper {
    pub const DOMAIN: &'static str = "
(define (domain gripper-strips)
    (:predicates 
        (room ?r)
		(ball ?b)
		(gripper ?g)
		(at-robby ?r)
		(at ?b ?r)
		(free ?g)
		(carry ?o ?g)
    )

    (:action move
        :parameters (?from ?to)
        :precondition (and 
            (room ?from) 
            (room ?to) 
            (at-robby ?from)
        )
        :effect (and  
            (at-robby ?to)
            (not (at-robby ?from))
        )
    )


    (:action pick
        :parameters (?obj ?room ?gripper)
        :precondition (and  
            (ball ?obj)
            (room ?room)
            (gripper ?gripper)
         	(at ?obj ?room)
            (at-robby ?room)
            (free ?gripper)
        )
        :effect (and
            (carry ?obj ?gripper)
		    (not (at ?obj ?room)) 
		    (not (free ?gripper))
        )
    )

    (:action drop
        :parameters (?obj ?room ?gripper)
        :precondition (and
            (ball ?obj)
            (room ?room)
            (gripper ?gripper)
			(carry ?obj ?gripper)
            (at-robby ?room)
        )
        :effect (and
            (at ?obj ?room)
		    (free ?gripper)
		    (not (carry ?obj ?gripper))
        )
    )
)
";
    pub const PROBLEM_UNSOLVED: &'static str = "
(define (problem strips-gripper4)
    (:domain gripper-strips)
    (:objects rooma roomb ball1 ball2 ball3 ball4 left right)
    (:init (room rooma)
           (room roomb)
           (ball ball1)
           (ball ball2)
           (ball ball3)
           (ball ball4)
           (gripper left)
           (gripper right)
           (at-robby rooma)
           (free left)
           (free right)
           (at ball1 rooma)
           (at ball2 rooma)
           (at ball3 rooma)
           (at ball4 rooma))
    (:goal (and 
        (at ball1 roomb)
        (at ball2 roomb)
        (at ball3 roomb)
        (at ball4 roomb))
    )
)";
    pub const PROBLEM_HALFSOLVED: &'static str = "
(define (problem strips-gripper4)
    (:domain gripper-strips)
    (:objects rooma roomb ball1 ball2 ball3 ball4 left right)
    (:init (room rooma)
           (room roomb)
           (ball ball1)
           (ball ball2)
           (ball ball3)
           (ball ball4)
           (gripper left)
           (gripper right)
           (at-robby rooma)
           (free left)
           (free right)
           (at ball1 rooma)
           (at ball2 rooma)
           (at ball3 roomb)
           (at ball4 roomb))
    (:goal (and 
        (at ball1 roomb)
        (at ball2 roomb)
        (at ball3 roomb)
        (at ball4 roomb))
    )
)";
    pub const PROBLEM_SOLVED: &'static str = "
(define (problem strips-gripper4)
    (:domain gripper-strips)
    (:objects rooma roomb ball1 ball2 ball3 ball4 left right)
    (:init (room rooma)
           (room roomb)
           (ball ball1)
           (ball ball2)
           (ball ball3)
           (ball ball4)
           (gripper left)
           (gripper right)
           (at-robby rooma)
           (free left)
           (free right)
           (at ball1 roomb)
           (at ball2 roomb)
           (at ball3 roomb)
           (at ball4 roomb)
    )
    (:goal (and 
        (at ball1 roomb)
        (at ball2 roomb)
        (at ball3 roomb)
        (at ball4 roomb))
    )
)";
}
