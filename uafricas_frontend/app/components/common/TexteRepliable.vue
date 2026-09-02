<script setup lang="ts">
/**
 * Texte long tronqué, avec ou sans commande de dépliage, feature 010.
 *
 * **Deux modes, et la distinction n'est pas cosmétique** :
 *
 *  • `repliable` (défaut) : pages de détail : le texte se déplie et se replie
 *    sur place, sans rechargement (FR-021).
 *  • `repliable="false"` : sections de vitrine : le texte est coupé par des
 *    points de suspension et s'arrête là (FR-003). La vitrine annonce, elle ne
 *    déroule pas ; c'est la page de détail qui déroule.
 *
 * Dans les deux modes, un texte qui tient entièrement s'affiche **sans bouton
 * et sans ellipse** (FR-022) : la détection se fait sur la hauteur réellement
 * rendue, jamais sur un décompte de caractères, qui dépendrait de la largeur.
 *
 * Tailwind v4 pur (Principe VI) : ce composant est monté sur des pages
 * publiques.
 */

const props = withDefaults(defineProps<{
  texte?: string | null
  /** Nombre de lignes visibles à l'état replié. */
  lignes?: number
  /** Fond sombre : les vitrines médias sont en noir. */
  sombre?: boolean
  /** `false` = ellipse figée, sans commande de dépliage. */
  repliable?: boolean
}>(), {
  texte: '',
  lignes: 4,
  sombre: false,
  repliable: true,
})

const deplie = ref(false)
const debordement = ref(false)
const corps = ref<HTMLElement | null>(null)

const texteNettoye = computed(() => (props.texte ?? '').trim())

/**
 * Le texte déborde-t-il de la troncature ?
 *
 * `scrollHeight > clientHeight` est mesuré **à l'état replié** : mesurer une
 * fois déplié rendrait toujours faux. Une marge d'un pixel absorbe les arrondis
 * de sous-pixel, qui feraient sinon apparaître un « voir plus » sur un texte
 * qui tient tout entier.
 */
const mesurer = () => {
  const el = corps.value
  if (!el || deplie.value) return
  debordement.value = el.scrollHeight - el.clientHeight > 1
}

let observateur: ResizeObserver | null = null

onMounted(async () => {
  await nextTick()
  mesurer()
  // La largeur change (rotation, redimensionnement) : ce qui tenait sur quatre
  // lignes peut cesser de tenir.
  if (typeof ResizeObserver !== 'undefined' && corps.value) {
    observateur = new ResizeObserver(() => mesurer())
    observateur.observe(corps.value)
  }
})

onBeforeUnmount(() => {
  observateur?.disconnect()
  observateur = null
})

watch(() => props.texte, async () => {
  deplie.value = false
  await nextTick()
  mesurer()
})

/**
 * Les deux propriétés préfixées sont écrites **en toutes lettres**, pas en
 * camelCase : Vue convertit `WebkitBoxOrient` en `webkit-box-orient`, sans le
 * tiret initial : une propriété que le navigateur ignore en silence. La
 * troncature n'avait alors aucun effet, et `scrollHeight === clientHeight`
 * faisait disparaître le bouton « voir plus » avec elle.
 */
const styleTronque = computed(() =>
  deplie.value
    ? {}
    : {
        'display': '-webkit-box',
        '-webkit-box-orient': 'vertical',
        '-webkit-line-clamp': String(props.lignes),
        'overflow': 'hidden',
      },
)

const classeBouton = computed(() =>
  props.sombre
    ? 'text-custom-chocolat hover:text-white'
    : 'text-custom-chocolat hover:text-custom-green',
)
</script>

<template>
  <div v-if="texteNettoye">
    <p
      ref="corps"
      class="whitespace-pre-line"
      :style="styleTronque"
    >{{ texteNettoye }}</p>

    <button
      v-if="repliable && (debordement || deplie)"
      type="button"
      class="mt-1 text-sm font-medium underline underline-offset-2 transition-colors cursor-pointer"
      :class="classeBouton"
      @click="deplie = !deplie"
    >
      {{ deplie ? 'voir moins' : 'voir plus' }}
    </button>
  </div>
</template>
