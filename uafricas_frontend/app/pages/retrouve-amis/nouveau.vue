<script setup lang="ts">
import gsap from 'gsap'
import { useAnimationsFormulaire } from '~/composables/useAnimationsFormulaire'

definePageMeta({ layout: false })

const userStore = useUserStore()
const { redirigerVersConnexion } = useAuth()
const { creerAvis } = useRetrouvAmis()

const chargement = ref(false)
const succes = ref(false)
const slugCree = ref('')
const correspondancesTrouvees = ref(0)
const erreur = ref('')

// ── Refs template pour animations ──────────────────────────────
const pageRef = ref<HTMLElement | null>(null)
const iconeSuccesRef = ref<HTMLElement | null>(null)
const titreSuccesRef = ref<HTMLElement | null>(null)
const messageCorrespondancesRef = ref<HTMLElement | null>(null)
const boutonsSuccesRef = ref<HTMLElement | null>(null)
const conteneurConfettisRef = ref<HTMLElement | null>(null)
const erreurRef = ref<HTMLElement | null>(null)

// ── Composable animations ──────────────────────────────────────
const {
  prefereReducedMotion,
  lancerConfettis,
  animerCompteur,
  animerErreur,
} = useAnimationsFormulaire(pageRef)

const onSoumettre = async (formData: FormData) => {
  chargement.value = true
  erreur.value = ''
  try {
    const res = await creerAvis(formData)
    if (res) {
      correspondancesTrouvees.value = res.correspondances_trouvees ?? 0
      slugCree.value = res.slug ?? ''
      succes.value = true
    }
  } catch (e: any) {
    erreur.value = e?.data?.error || e?.message || 'Une erreur est survenue lors de la soumission.'
  } finally {
    chargement.value = false
  }
}

const onAnnuler = () => {
  navigateTo('/retrouve-amis')
}

// ── Watch succes → celebration (US4) ───────────────────────────
watch(succes, async (valeur) => {
  if (!valeur) return
  await nextTick()

  if (prefereReducedMotion.value) return

  const tl = gsap.timeline()

  // 1. Bounce-in icone
  if (iconeSuccesRef.value) {
    tl.fromTo(iconeSuccesRef.value,
      { scale: 0, opacity: 0 },
      { scale: 1, opacity: 1, duration: 0.5, ease: 'back.out(1.4)' },
    )
  }

  // 2. Fade-in titre
  if (titreSuccesRef.value) {
    tl.fromTo(titreSuccesRef.value,
      { y: 20, opacity: 0 },
      { y: 0, opacity: 1, duration: 0.4 },
      '-=0.1',
    )
  }

  // 3. Confettis
  lancerConfettis(conteneurConfettisRef)

  // 4. Compteur correspondances
  if (correspondancesTrouvees.value > 0 && messageCorrespondancesRef.value) {
    tl.fromTo(messageCorrespondancesRef.value,
      { opacity: 0, y: 10 },
      { opacity: 1, y: 0, duration: 0.3 },
      '-=0.2',
    )
    const compteurEl = messageCorrespondancesRef.value.querySelector('.compteur-correspondances')
    if (compteurEl) {
      animerCompteur(ref(compteurEl as HTMLElement), correspondancesTrouvees.value)
    }
  }

  // 5. Stagger boutons
  if (boutonsSuccesRef.value) {
    const boutons = boutonsSuccesRef.value.children
    tl.fromTo(boutons,
      { opacity: 0, y: 15, scale: 0.9 },
      { opacity: 1, y: 0, scale: 1, duration: 0.3, stagger: 0.15, ease: 'back.out(1.2)' },
      '-=0.1',
    )
  }
})

// ── Watch erreur → shake (US4) ─────────────────────────────────
watch(erreur, async (valeur) => {
  if (!valeur) return
  await nextTick()
  animerErreur(erreurRef)
})

onMounted(() => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
  }
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <!-- L'image était HOTLINKÉE sur Unsplash : hors de notre contrôle, elle
           pouvait disparaître ou changer sans préavis. Le hero local du module
           existait déjà, il est simplement repris. -->
      <AfricansBandeauModule
        titre="Nouvel avis de recherche"
        sous-titre="Décrivez la personne que vous recherchez pour lancer la mise en relation."
        image="/images/africans/heros/hero-africonnect.jpg"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Opafrica', vers: '/retrouve-amis' },
          { libelle: 'Africonnect', vers: '/retrouve-amis' },
          { libelle: 'Nouvel avis' },
        ]"
      />
    </template>

    <div ref="pageRef">
      <!-- Confirmation -->
      <div
        v-if="succes"
        class="relative overflow-hidden rounded-[10px] border border-af-vert/30 bg-white p-8 text-center"
      >
        <div ref="conteneurConfettisRef" class="pointer-events-none absolute inset-0 overflow-hidden" />

        <div
          ref="iconeSuccesRef"
          class="mx-auto mb-4 grid size-16 place-items-center rounded-full bg-af-vert/10 text-af-vert"
        >
          <font-awesome-icon icon="fa-solid fa-check" class="text-3xl" />
        </div>
        <h2 ref="titreSuccesRef" class="mb-2 text-[20px]/[1.4] font-bold text-af-encre">
          Avis de recherche publié !
        </h2>
        <p class="text-[14px]/[1.6] text-af-corps">
          Votre avis est maintenant visible par tous les visiteurs.
        </p>
        <p
          v-if="correspondancesTrouvees > 0"
          ref="messageCorrespondancesRef"
          class="mt-2 text-[14px]/[1.4] font-bold text-af-vert"
        >
          <span class="compteur-correspondances">{{ correspondancesTrouvees }}</span>
          correspondance(s) potentielle(s) trouvée(s) !
        </p>
        <p v-else class="mt-2 text-[14px]/[1.4] text-af-atone">
          Nous vous notifierons dès qu'une correspondance sera trouvée.
        </p>

        <div ref="boutonsSuccesRef" class="mt-6 flex flex-wrap justify-center gap-3">
          <AfricansBouton
            v-if="slugCree"
            :vers="`/retrouve-amis/public/${slugCree}`"
            icone="fa-solid fa-eye"
          >
            Voir votre avis tel que publié
          </AfricansBouton>
          <AfricansBouton vers="/retrouve-amis/mes-recherches" variante="secondaire">
            Mes recherches
          </AfricansBouton>
        </div>
      </div>

      <!-- Formulaire -->
      <div v-else class="flex flex-col gap-6">
        <p
          v-if="erreur"
          ref="erreurRef"
          class="flex items-center gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
        >
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="shrink-0" />
          {{ erreur }}
        </p>

        <RetrouveAmisAvisRechercheForm
          mode="creation"
          :chargement-soumission="chargement"
          @submit="onSoumettre"
          @annuler="onAnnuler"
        />
      </div>
    </div>

    <template #rail>
      <RetrouveAmisSideBar />
    </template>
  </NuxtLayout>
</template>
