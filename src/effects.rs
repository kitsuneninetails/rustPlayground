extern crate num_traits;

use num_traits::*;

trait F<X> {}

trait Semigroup<X> {
    fn combine(a: X, b: X) -> X;
}

trait Monoid {
    fn empty() -> Self;
}

trait Functor<FX, FY, X, Y>
    where FX: F<X>,
          FY: F<Y> {
    fn fmap(f: FX, func: fn(X) -> Y) -> FY;
}

trait Functor2<FX, FY, FZ, X, Y, Z>
    where FX: F<X>,
          FY: F<Y>,
          FZ: F<Z> {
    fn fmap2(fa: FX, fb: FY, func: fn(&X, &Y) -> Z) -> FZ;
}

trait Productable<FX, FY, FXY, X, Y>
    where FX: F<X>,
          FY: F<Y>,
          FXY: F<(X, Y)> {
    fn product(fa: FX, fb: FY) -> FXY;
}

trait Applicative<FX, X> {
    fn pure(x: X) -> FX;
}

trait Monad<FX, FY, X, Y>
    where FX: F<X>,
          FY: F<Y> {
    fn flat_map(f: FX, func: fn(X) -> FY) -> FY;
}

trait Foldable<FX, X, Y>
    where FX: F<X> {
    fn fold(f: FX, init: Y, func: impl Fn(Y, X) -> Y) -> Y;
}

trait Traverse<FX, FY, AY, AR, X, Y>
    where FX: F<X>,
          FY: F<Y>,
          AY: F<Y> + Applicative<AY, Y>,
          AR: F<FY> + Applicative<AR, FY>
{
    fn traverse(f: FX, func: fn(X) -> AY) -> AR;
}

//impl<Y: Num> Semigroup for Y {
//    fn combine<X: Num>(a: X, b: X) -> X {
//        a + b
//    }
//}

impl Semigroup<String> for String {
    fn combine(a: String, b: String) -> String {
        format!("{}{}", a, b)
    }
}

impl<X> F<X> for Option<X> {}

impl<X> Semigroup<Option<X>> for Option<X> where X: Semigroup<X> {
    fn combine(a: Option<X>, b: Option<X>) -> Option<X> {
        a.and_then(|i| b.map(|j| X::combine(i, j)))
    }
}

impl<X> Monoid for Option<X> {
    fn empty() -> Self { None }
}

impl<X, Y> Functor<Option<X>, Option<Y>, X, Y> for Option<X> {
    fn fmap(f: Option<X>, func: fn(X) -> Y) -> Option<Y> {
        f.map(func)
    }
}

impl<X, Y, Z> Functor2<Option<X>, Option<Y>, Option<Z>, X, Y, Z> for Option<X> {
    fn fmap2(fa: Option<X>, fb: Option<Y>, func: fn(&X, &Y) -> Z) -> Option<Z> {
        fa.and_then(|i| fb.map(|j| func(&i, &j)))
    }
}

impl<X: Clone, Y: Clone> Productable<Option<X>, Option<Y>, Option<(X, Y)>, X, Y> for Option<X> {
    fn product(fa: Option<X>, fb: Option<Y>) -> Option<(X,Y)> {
        Option::fmap2(fa, fb, |a, b| (a.clone(), b.clone()))
    }

}

impl<X> Applicative<Option<X>, X> for Option<X> {
    fn pure(x: X) -> Option<X> {
        Some(x)
    }
}

impl<X, Y> Monad<Option<X>, Option<Y>, X, Y> for Option<X> {
    fn flat_map(f: Option<X>, func: fn(X) -> Option<Y>) -> Option<Y> {
        f.and_then(func)
    }
}

impl<X, Y> Foldable<Option<X>, X, Y>  for Option<X> {
    fn fold(f: Option<X>, init: Y, func: impl Fn(Y, X) -> Y) -> Y {
        match f {
            Some(i) => func(init, i),
            None => init
        }
    }
}

impl<X> F<X> for Vec<X> {}

impl<X> Semigroup<Vec<X>> for Vec<X> {
    fn combine(a: Vec<X>, b: Vec<X>) -> Vec<X> {
        let mut ret = a;
        ret.extend(b);
        ret
    }
}

impl<X> Monoid for Vec<X> {
    fn empty() -> Self { vec![] }
}

impl<X, Y> Functor<Vec<X>, Vec<Y>, X, Y> for Vec<X> {
    fn fmap(f: Vec<X>, func: fn(X) -> Y) -> Vec<Y> {
        f.into_iter().map(func).collect()
    }
}

impl<X, Y, Z> Functor2<Vec<X>, Vec<Y>, Vec<Z>, X, Y, Z> for Vec<X> {
    fn fmap2(fa: Vec<X>, fb: Vec<Y>, func: fn(&X, &Y) -> Z) -> Vec<Z> {
        fa.into_iter().flat_map(|i|{
            let ret: Vec<Z> = fb.iter().map(|j| {
                func(&i, j)
            }).collect();
            ret
        }).collect()
    }
}

impl<X: Clone, Y: Clone> Productable<Vec<X>, Vec<Y>, Vec<(X, Y)>, X, Y> for Vec<X> {
    fn product(fa: Vec<X>, fb: Vec<Y>) -> Vec<(X,Y)> {
        Vec::fmap2(fa, fb, |a, b| (a.clone(), b.clone()))
    }
}

impl<X> Applicative<Vec<X>, X> for Vec<X> {
    fn pure(x: X) -> Vec<X> {
        vec![x]
    }
}

impl<X, Y> Monad<Vec<X>, Vec<Y>, X, Y> for Vec<X> {
    fn flat_map(f: Vec<X>, func: fn(X) -> Vec<Y>) -> Vec<Y> {
        f.into_iter().flat_map(func).collect()
    }
}

impl<X, Y> Foldable<Vec<X>, X, Y> for Vec<X> {
    fn fold(f: Vec<X>, init: Y, func: impl Fn(Y, X) -> Y) -> Y {
        let mut accum = init;
        for i in f.into_iter() {
            accum = func(accum, i);
        }
        accum
    }
}

//implicit val traverseForList: Traverse[List] = new Traverse[List] {
//  def traverse[G[_]: Applicative, A, B](fa: List[A])(f: A => G[B]): G[List[B]] =
//      fa.foldRight(Applicative[G].pure(List.empty[B]))
//      { (a, acc) =>
//          Applicative[G].map2(f(a), acc)(_ :: _)
//      }
//}

//def traverse(as: List[X])(f: X => G): H =
//  as.foldRight(Applicative[F].pure(List.empty[Y]))
//  { (a: X, acc: H) =>
//      val fb: G = f(a)
//      Applicative[Vec<?>].map2(fb, acc)(_ :: _)
//  }


impl<X, Y: Clone, AY, AR> Traverse<Vec<X>, Vec<Y>, AY, AR, X, Y> for Vec<X>
    where AY: F<Y> + Applicative<AY, Y> + Functor2<AY, AR, AR, Y, Vec<Y>, Vec<Y>>,
          AR: F<Vec<Y>> + Applicative<AR, Vec<Y>> {
    fn traverse(fa: Vec<X>, func: fn(X) -> AY) -> AR {
        // Make an empty list of Y where Y is whatever the Applicative (Option, Future, etc.)
        // is set to hold after the function.  This is used to kick start the contained vector
        // which will hold the resulting values (and will be contained by the specified
        // Applicative (Future, Option, Etiher, etc.))
        let empty_ret_list = Vec::<Y>::empty();

        // Fold on the initial list (Vec<X>) and start with initial accumulator set to
        // A basic G<Vec<Y>> where G is the Applicative that will be returned from the specified
        // function (Option, Future, Either, etc.).
        let init = AR::pure(empty_ret_list);
        Vec::fold(
            fa,
            init,
            |acc, item| {
                // The folding function should take this Applicative (Option, Future, etc.) and
                // "combine" the results with the accumulated value.  This is what determines
                // whether the accumulated value turns into a "negative" result (like a None,
                // or a Future::fail(), or a Either::Err, etc.)

                // First, get the returned Applicative from the func call:
                let ret_ay = func(item);

                // Apply a map between the returned value and the accumulated value.  The
                // mapping function should know how to put the two together (they are the same
                // Applicative type, but they each hold a different type inside).
                AY::fmap2(
                    ret_ay,
                    acc,
                    |fx, y| {
                        // This function adds the returned inner value onto the accumulating list
                        // inside the Applicative.  Applicatives know how to only allow this
                        // combination if both the accumulated Applicative and the returned
                        // Applicative both match up to "positive" values (like success or Some()).
                        // These next lines won't even get called unless that is the case.
                        let mut r = vec![fx.clone()];
                        r.extend(y.clone());
                        r
                    })
            })
    }
}

pub fn main() {
    let o1 = Some(1);
    let o2 = Option::fmap(o1, |x| x + 1);
    println!("O2 = {:?}", o2);
    let o3 = Option::fmap(o2, |x| format!("{}", x));
    println!("O3 = {:?}", o3);
    let o4 = Option::flat_map(Option::pure(1), |x| Some(format!("{}", x)));
    println!("O4 = {:?}", o4);
    let o5 = Option::fmap2(Option::pure(1), Option::pure("tons".to_string()),
                           |x, y| format!("{} {}", x, y));
    println!("O5 = {:?}", o5);

    let v1 = vec![1];
    let v2 = Vec::fmap(v1, |x| x + 1);
    println!("V2 = {:?}", v2);
    let v3 = Vec::fmap(v2, |x| format!("{}", x));
    println!("V3 = {:?}", v3);
    let v4 = Vec::flat_map(vec![1, 2], |x| vec!["V".to_string(), (format!("{}", x))]);
    println!("V4 = {:?}", v4);
    let v5 = Vec::fmap2(vec![1, 2], vec![3, 4], |x, y| x + y);
    println!("V5 = {:?}", v5);

    let v6 = vec![2, 4, 6];
    let v6b = Vec::traverse(v6, |x| match x % 2 == 0 {
        true => Some(format!("{}", x)),
        false => None
    });
    println!("V6 = {:?}", v6b); // Some(Vec("2", "4", "6"))

    let v7 = vec![2, 5, 6];
    let v7b = Vec::traverse(v7, |x| match x % 2 == 0 {
        true => Some(format!("{}", x)),
        false => None
    });
    println!("V5 = {:?}", v7b); // None

}
