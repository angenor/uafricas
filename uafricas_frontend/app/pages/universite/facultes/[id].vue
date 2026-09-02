<script setup lang="ts">
import { useFacultes, type Faculte } from '~/composables/useFacultes'

/**
 * Fiche d'une faculté partenaire, portée sur le gabarit de la refonte.
 *
 * Le repli sur les mocks est SUPPRIMÉ. En cas d'échec de l'API, la page
 * affichait une faculté INVENTÉE — programmes, frais de scolarité, effectifs —
 * sans rien signaler au visiteur, et un `console.warn` que personne ne lit.
 * Une panne doit se dire ; elle ne doit pas se combler par de la fiction.
 *
 * L'`alert()` de « manifester son intérêt » laisse place à une modale qui
 * énonce ce qui est : aucun endpoint ne reçoit encore cette manifestation.
 */
definePageMeta({ layout: false })

const route = useRoute()
const { loading, obtenirFaculte } = useFacultes()

const faculte = ref<Faculte | null>(null)
const erreur = ref<string | null>(null)
const interetOuvert = ref(false)

const charger = async () => {
  erreur.value = null
  try {
    faculte.value = await obtenirFaculte(route.params.id as string)
  }
  catch (e: any) {
    faculte.value = null
    erreur.value = e?.message || 'Impossible de charger cette faculté pour le moment.'
  }
}

onMounted(charger)

useHead({ title: computed(() => faculte.value ? `${faculte.value.titre} | Muniversa` : 'Faculté | Muniversa') })

const groupesProgrammes = computed(() => {
  const p = faculte.value?.programmesResume
  if (!p) return []
  return [
    { titre: 'Licences', items: p.licence },
    { titre: 'Masters', items: p.master },
    { titre: 'Doctorats', items: p.doctorat },
    { titre: 'Certificats', items: p.certificats },
  ].filter(g => g.items?.length)
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        :titre="faculte?.titre ?? 'Faculté'"
        :sous-titre="faculte?.acronyme"
        :image="faculte?.imageCouverture || '/images/education.png'"
      >
        <template v-if="faculte?.accepteNouveauxInscrits" #action>
          <span class="rounded-lg bg-af-vert px-4 py-2 text-[14px]/[1.4] font-bold text-white">
            Inscriptions ouvertes
          </span>
        </template>
      </AfricansBandeauModule>
    </template>

    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Muniversa', vers: '/universite' },
          { libelle: 'Facultés', vers: '/universite/facultes' },
          { libelle: faculte?.titre ?? 'Faculté' },
        ]"
      />
    </template>

    <div v-if="loading" class="flex flex-col gap-6">
      <div v-for="n in 2" :key="n" class="h-48 animate-pulse rounded-[10px] bg-af-bordure" />
    </div>

    <!-- Une panne se dit. Elle ne se comble pas par une faculté inventée. -->
    <div v-else-if="!faculte" class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
      <font-awesome-icon icon="fa-solid fa-building-columns" class="text-4xl text-af-atone-2" />
      <p class="mt-4 text-[16px]/[1.4] font-bold">
        {{ erreur ? 'Cette faculté n’a pas pu être chargée' : 'Faculté introuvable' }}
      </p>
      <p class="mx-auto mt-2 max-w-md text-[14px]/[1.4] text-af-corps">
        {{ erreur ?? 'La faculté que vous recherchez n’existe pas ou n’est plus partenaire.' }}
      </p>
      <div class="mt-6 flex flex-wrap justify-center gap-3">
        <AfricansBouton v-if="erreur" icone="fa-solid fa-rotate-right" @click="charger">Réessayer</AfricansBouton>
        <AfricansBouton variante="secondaire" icone="fa-solid fa-arrow-left" vers="/universite/facultes">
          Retour aux facultés
        </AfricansBouton>
      </div>
    </div>

    <div v-else class="flex flex-col gap-6">
      <AfricansAccordeon titre="À propos" icone="fa-solid fa-circle-info" par-defaut-ouvert>
        <p class="text-[14px]/[1.6] whitespace-pre-line text-af-corps">{{ faculte.description }}</p>
      </AfricansAccordeon>

      <AfricansAccordeon v-if="groupesProgrammes.length" titre="Programmes proposés" icone="fa-solid fa-graduation-cap" par-defaut-ouvert>
        <div class="grid gap-5 sm:grid-cols-2">
          <div v-for="groupe in groupesProgrammes" :key="groupe.titre">
            <h3 class="mb-2 text-[14px]/[1.4] font-bold text-af-chocolat">{{ groupe.titre }}</h3>
            <ul class="flex flex-col gap-1">
              <li v-for="prog in groupe.items" :key="prog" class="flex gap-2 text-[14px]/[1.5] text-af-corps">
                <font-awesome-icon icon="fa-solid fa-circle" class="mt-2 size-1.5 shrink-0 text-af-atone-2" />
                {{ prog }}
              </li>
            </ul>
          </div>
        </div>
      </AfricansAccordeon>

      <AfricansAccordeon v-if="faculte.pointsForts?.length" titre="Points forts" icone="fa-solid fa-star">
        <ul class="flex flex-col gap-2">
          <li v-for="point in faculte.pointsForts" :key="point" class="flex gap-3 text-[14px]/[1.5] text-af-corps">
            <font-awesome-icon icon="fa-solid fa-circle-check" class="mt-1 size-4 shrink-0 text-af-vert" />
            {{ point }}
          </li>
        </ul>
      </AfricansAccordeon>

      <AfricansAccordeon v-if="faculte.domainesEtudes?.length" titre="Domaines d'études" icone="fa-solid fa-book">
        <div class="flex flex-wrap gap-2">
          <AfricansEtiquette v-for="d in faculte.domainesEtudes" :key="d">{{ d }}</AfricansEtiquette>
        </div>
      </AfricansAccordeon>
    </div>

    <template #rail>
      <template v-if="faculte">
        <AfricansPanneau v-if="faculte.ecolePartenaire" titre="École partenaire" icone="fa-solid fa-building-columns">
          <p class="text-[14px]/[1.4] font-bold text-af-encre">{{ faculte.ecolePartenaire.nom }}</p>
          <p class="mt-1 text-[14px]/[1.4] text-af-corps">
            {{ faculte.ecolePartenaire.ville }}, {{ faculte.ecolePartenaire.pays }}
          </p>
          <AfricansEtiquette class="mt-3" :ton="faculte.ecolePartenaire.type === 'publique' ? 'vert' : 'gris'">
            {{ faculte.ecolePartenaire.type === 'publique' ? 'Public' : 'Privé' }}
          </AfricansEtiquette>
          <a
            v-if="faculte.ecolePartenaire.siteWeb"
            :href="faculte.ecolePartenaire.siteWeb"
            target="_blank"
            rel="noopener noreferrer"
            class="mt-4 flex items-center gap-2 text-[14px]/[1.4] font-bold text-af-chocolat transition hover:opacity-70"
          >
            <font-awesome-icon icon="fa-solid fa-link" />
            Site de l'établissement
          </a>
        </AfricansPanneau>

        <AfricansPanneau titre="Conditions d'admission" icone="fa-solid fa-clipboard-list">
          <dl class="flex flex-col gap-3">
            <div v-if="faculte.conditionsAdmission?.diplomeMinimum">
              <dt class="text-[12px]/[1.4] text-af-atone">Diplôme minimum</dt>
              <dd class="text-[14px]/[1.4] font-bold">{{ faculte.conditionsAdmission.diplomeMinimum }}</dd>
            </div>
            <div v-if="faculte.conditionsAdmission?.languesEnseignement?.length">
              <dt class="text-[12px]/[1.4] text-af-atone">Langues d'enseignement</dt>
              <dd class="text-[14px]/[1.4] font-bold">{{ faculte.conditionsAdmission.languesEnseignement.join(', ') }}</dd>
            </div>
            <div v-if="faculte.conditionsAdmission?.fraisScolariteAnnuels?.max">
              <dt class="text-[12px]/[1.4] text-af-atone">Frais annuels</dt>
              <dd class="text-[14px]/[1.4] font-bold">
                {{ faculte.conditionsAdmission.fraisScolariteAnnuels.min?.toLocaleString('fr-FR') }}
                à {{ faculte.conditionsAdmission.fraisScolariteAnnuels.max.toLocaleString('fr-FR') }} FCFA
              </dd>
              <dd v-if="faculte.conditionsAdmission.fraisScolariteAnnuels.boursesPossibles" class="text-[12px]/[1.4] text-af-vert">
                Bourses possibles
              </dd>
            </div>
            <div v-if="faculte.conditionsAdmission?.periodesInscription">
              <dt class="text-[12px]/[1.4] text-af-atone">Périodes d'inscription</dt>
              <dd class="text-[14px]/[1.4] font-bold">{{ faculte.conditionsAdmission.periodesInscription }}</dd>
            </div>
          </dl>
        </AfricansPanneau>

        <AfricansPanneau titre="Effectifs" icone="fa-solid fa-users">
          <dl class="flex flex-col">
            <div class="flex items-baseline justify-between gap-4 pb-3">
              <dt class="text-[14px]/[1.4] font-bold">Inscrits au total</dt>
              <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ faculte.stats?.nombreInscritsTotal ?? 0 }}</dd>
            </div>
            <div class="flex items-baseline justify-between gap-4 border-t border-af-bordure pt-3">
              <dt class="text-[14px]/[1.4] font-bold">Cette année</dt>
              <dd class="text-[14px]/[1.4] font-bold text-af-chocolat">{{ faculte.stats?.nombreInscritsAnneeEnCours ?? 0 }}</dd>
            </div>
          </dl>
        </AfricansPanneau>

        <AfricansPanneau titre="Candidater" icone="fa-solid fa-paper-plane">
          <AfricansBouton
            pleine-largeur
            icone="fa-solid fa-paper-plane"
            :desactive="!faculte.accepteNouveauxInscrits"
            @click="interetOuvert = true"
          >
            {{ faculte.accepteNouveauxInscrits ? 'Manifester mon intérêt' : 'Inscriptions fermées' }}
          </AfricansBouton>
        </AfricansPanneau>
      </template>
    </template>

    <AfricansModale
      v-model="interetOuvert"
      titre="Manifester mon intérêt"
      icone="fa-solid fa-paper-plane"
    >
      <p v-if="faculte" class="text-[14px]/[1.5] text-af-corps">
        Vous manifestez votre intérêt pour
        <strong class="font-bold text-af-encre">{{ faculte.titre }}</strong>.
      </p>
      <!-- Dit ce qui est : aucun endpoint ne reçoit encore cette manifestation.
           Un `alert()` disait la même chose, mais sans laisser de porte de sortie. -->
      <p class="mt-3 text-[14px]/[1.4] text-af-atone">
        La candidature en ligne n'est pas encore ouverte. En attendant, contactez directement
        l'établissement partenaire.
      </p>

      <template #actions>
        <AfricansBouton @click="interetOuvert = false">J'ai compris</AfricansBouton>
      </template>
    </AfricansModale>
  </NuxtLayout>
</template>
