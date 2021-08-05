//#[warn(dead_code)] // This is the default warning
#[allow(dead_code)] // We use this to tell the compiler to stop warning the unused code
pub fn merge_sort(v: &mut [i32], start: usize, end: usize) {
    if end-start > 1 {
        let middle = (start+end)/2;
        merge_sort(v, start, middle);
        merge_sort(v, middle, end);

        let mut aux_j = vec![0; middle-start]; // To create an empty vector, just use vec![]
        let mut aux_k = vec![0; end-middle];

        /*for i in 0..(middle-start) { // Encontrar uma melhor forma de copiar os dados
            aux_j[i] = v[i+start];
        }
        for i in 0..(end-middle) {
            aux_k[i] = v[middle+i];
        }*/

        aux_j.copy_from_slice(&v[start..middle]); // Encontrado a melhor forma
        aux_k.copy_from_slice(&v[middle..end]);

        let mut j = 0;
        let mut k = 0;
        let mut index_i: usize = 0;
        for i in start..end { // Aprender a colocar condições no for
            if j >= middle-start || k >= end-middle {
                break;
            }

            if aux_j[j] < aux_k[k] {
                v[i] = aux_j[j];
                j += 1;
            } else {
                v[i] = aux_k[k];
                k += 1;
            }
            index_i = i;
        }
        index_i += 1;

        for p in j..(middle-start) {
            v[index_i] = aux_j[p];
            index_i += 1;
        }

        for p in k..(end-middle) {
            v[index_i] = aux_k[p];
            index_i += 1;
        }
    }
}

#[allow(dead_code)]
pub fn bubble_sort(v: &mut [i32], start: usize, end: usize) {
    let mut finish = false;
    for _ in start..end {
        if finish { break; }
        finish = true;
        for i in start..(end-1) {
            if v[i] > v[i+1] {
                let aux = v[i];
                v[i] = v[i+1];
                v[i+1] = aux;
                finish = false;
            }
        }
    }
}

#[allow(dead_code)]
pub fn selection_sort(v: &mut [i32], start: usize, end: usize) {
    let mut j = start;
    let mut k = end-1;
    for _ in start..(end/2) {
        let mut greater = v[j];
        let mut smallest = v[k];
        let mut greater_index = j;
        let mut smallest_index = k;
        for i in j..k+1 {
            if v[i] > greater {
                greater_index = i;
                greater = v[i];
            } else if v[i] < smallest {
                smallest_index = i;
                smallest = v[i];
            }
        }
        let tmp = v[j];
        v[j] = v[smallest_index];
        v[smallest_index] = tmp;

        let tmp = v[k];
        v[k] = v[greater_index];
        v[greater_index] = tmp;

        j += 1;
        k -= 1;
    }
}