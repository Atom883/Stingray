import { atom } from 'jotai';

export const userDataAtom = atom({
    userId: '',
    aState: {
        name: '',
        hp: 50,
        maxHp: 111,
        color: "red",
        font: "Arial, sans-serif",
        isBold: true,
        isOutlined: true,
    },
    feeds: {},
});